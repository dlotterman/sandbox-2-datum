"""Tests for unpack-image.py. Run:  python3 -m unittest discover -s deploy/vm"""
import hashlib
import importlib.util
import io
import json
import os
import tarfile
import tempfile
import unittest

spec = importlib.util.spec_from_file_location("unpack", os.path.join(os.path.dirname(__file__), "unpack-image.py"))
unpack = importlib.util.module_from_spec(spec)
spec.loader.exec_module(unpack)


class FakeResponse(io.BytesIO):
    def __init__(self, body, content_type="application/vnd.oci.image.manifest.v1+json"):
        super().__init__(body)
        self.headers = {"Content-Type": content_type}


class FakeRegistry:
    """Serves canned bytes for manifest and blob paths."""

    def __init__(self, files):
        self.files = files

    def get(self, path, accept=None):
        return FakeResponse(self.files[path])


def layer_with(name, content=b"binary"):
    """A gzip tarball containing one executable file, and its sha256 digest."""
    buffer = io.BytesIO()
    with tarfile.open(fileobj=buffer, mode="w:gz") as archive:
        info = tarfile.TarInfo(name)
        info.size, info.mode = len(content), 0o755
        archive.addfile(info, io.BytesIO(content))
    blob = buffer.getvalue()
    return blob, "sha256:" + hashlib.sha256(blob).hexdigest()


class UnpackTests(unittest.TestCase):
    def test_references_parse_with_digest_tag_or_neither(self):
        digest = "sha256:" + "a" * 64
        self.assertEqual(unpack.parse_reference(f"ghcr.io/o/r@{digest}"), ("ghcr.io", "o/r", digest))
        self.assertEqual(unpack.parse_reference("ghcr.io/o/r:v1"), ("ghcr.io", "o/r", "v1"))
        self.assertEqual(unpack.parse_reference("ghcr.io/o/r"), ("ghcr.io", "o/r", "latest"))

    def test_a_manifest_that_does_not_hash_to_the_pinned_digest_is_refused(self):
        body = json.dumps({"layers": []}).encode()
        registry = FakeRegistry({"/manifests/sha256:" + "b" * 64: body})
        with self.assertRaises(SystemExit) as refused:
            unpack.fetch_manifest(registry, "sha256:" + "b" * 64)
        self.assertIn("does not hash", str(refused.exception))

    def test_a_manifest_that_matches_its_digest_is_accepted(self):
        body = json.dumps({"layers": [{"digest": "x"}]}).encode()
        digest = "sha256:" + hashlib.sha256(body).hexdigest()
        manifest = unpack.fetch_manifest(FakeRegistry({f"/manifests/{digest}": body}), digest)
        self.assertEqual(manifest["layers"][0]["digest"], "x")

    def test_the_file_is_extracted_with_its_mode_and_other_files_are_ignored(self):
        blob, digest = layer_with("./usr/local/bin/agent", b"hello")
        registry = FakeRegistry({f"/blobs/{digest}": blob})
        with tempfile.TemporaryDirectory() as folder:
            target = os.path.join(folder, "bin", "agent")
            self.assertTrue(unpack.find_in_layer(registry, {"digest": digest, "mediaType": "x.tar+gzip"}, "usr/local/bin/agent", target))
            self.assertEqual(open(target, "rb").read(), b"hello")
            self.assertEqual(os.stat(target).st_mode & 0o777, 0o755)
            self.assertFalse(os.path.exists(target + ".partial"), "no half-written file is left behind")
            self.assertFalse(unpack.find_in_layer(registry, {"digest": digest, "mediaType": "x.tar+gzip"}, "etc/passwd", target))

    def test_a_layer_that_does_not_hash_to_its_digest_is_refused_and_nothing_is_written(self):
        blob, digest = layer_with("usr/local/bin/agent")
        registry = FakeRegistry({f"/blobs/{digest}": blob + b"tampered"})
        with tempfile.TemporaryDirectory() as folder:
            target = os.path.join(folder, "agent")
            with self.assertRaises(SystemExit) as refused:
                unpack.find_in_layer(registry, {"digest": digest, "mediaType": "x.tar+gzip"}, "usr/local/bin/agent", target)
            self.assertIn("corrupted", str(refused.exception))
            self.assertFalse(os.path.exists(target))


if __name__ == "__main__":
    unittest.main()
