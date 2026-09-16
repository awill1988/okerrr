import unittest
from unittest.mock import patch
from urllib.error import HTTPError, URLError

from release_gate import (
    compare_versions,
    published_versions,
    release_decision,
    sparse_index_path,
)


class ReleaseGateTests(unittest.TestCase):
    def test_semver_prerelease_precedence(self):
        self.assertLess(compare_versions("0.2.0-beta.2", "0.2.0-beta.11"), 0)
        self.assertLess(compare_versions("0.2.0-beta.11", "0.2.0"), 0)
        self.assertGreater(compare_versions("0.2.0", "0.1.9"), 0)

    def test_build_metadata_does_not_increase_precedence(self):
        self.assertEqual(compare_versions("0.2.0+build.2", "0.2.0+build.1"), 0)

    def test_sparse_index_path(self):
        self.assertEqual(sparse_index_path("okerrr"), "ok/er/okerrr")

    def test_unchanged_version_skips_release(self):
        self.assertFalse(release_decision("0.0.0", "0.0.0", "0.1.0", []))

    def test_setup_change_establishes_unreleased_baseline(self):
        self.assertFalse(release_decision("0.0.0", "0.1.0", "0.1.0", []))

    def test_first_version_bump_releases(self):
        self.assertTrue(release_decision("0.0.1", "0.0.0", "0.1.0", []))

    def test_first_prerelease_bump_releases(self):
        self.assertTrue(release_decision("0.1.0-beta.1", "0.0.0", "0.1.0", []))

    def test_decrease_fails(self):
        with self.assertRaises(ValueError):
            release_decision("0.1.0", "0.2.0", "0.1.0", [])

    def test_baseline_reset_after_publish_fails(self):
        with self.assertRaises(ValueError):
            release_decision("0.0.0", "0.1.0", "0.1.0", ["0.1.0"])

    def test_baseline_reset_from_later_main_version_fails(self):
        with self.assertRaises(ValueError):
            release_decision("0.0.0", "0.2.0", "0.1.0", [])

    def test_published_duplicate_fails_even_if_yanked(self):
        with self.assertRaises(ValueError):
            release_decision("0.2.0", "0.1.0", "0.1.0", ["0.1.0", "0.2.0"])

    def test_registry_version_ahead_fails(self):
        with self.assertRaises(ValueError):
            release_decision("0.2.0", "0.1.0", "0.1.0", ["0.1.0", "0.3.0-beta.1"])

    def test_unpublished_prior_bump_fails(self):
        with self.assertRaises(ValueError):
            release_decision("0.3.0", "0.2.0", "0.1.0", ["0.1.0"])

    def test_missing_crate_has_no_published_versions(self):
        with patch(
            "release_gate.urlopen",
            side_effect=HTTPError("https://index.crates.io/ok/er/okerrr", 404, "", {}, None),
        ):
            self.assertEqual(published_versions("okerrr"), [])

    def test_registry_error_fails_closed(self):
        with patch("release_gate.urlopen", side_effect=URLError("unavailable")):
            with self.assertRaises(RuntimeError):
                published_versions("okerrr")


if __name__ == "__main__":
    unittest.main()
