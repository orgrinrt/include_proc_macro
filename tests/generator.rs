//! The generator regenerates the matrix, and survives doing so.
//!
//! `arm_matrix/` and `arm_matrix_test/src/` are 106 generated files, and the script that
//! writes them removes both trees first. It used to sit inside `arm_matrix/`, so the first
//! run deleted the script: the sole means of regenerating the matrix was destroyed by the
//! first regeneration, and the tree survived only because nobody had run it since.
//!
//! The comment at the top of that script already named the class, one line above the call
//! that did it. Naming a class and leaving its instance standing is the failure this file
//! exists to stop repeating.

use std::process::Command;

/// The repository root, which is where the generator expects to be run from.
const ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// Runs the generator and returns what it printed.
fn generate() -> String {
    let output = Command::new("python3")
        .arg("scripts/generate_arm_matrix.py")
        .current_dir(ROOT)
        .output()
        .expect("python3 runs");

    assert!(
        output.status.success(),
        "the generator exited {}\n--- stderr\n{}",
        output.status,
        String::from_utf8_lossy(&output.stderr),
    );

    String::from_utf8_lossy(&output.stdout).to_string()
}

/// Every file under the generated trees, with its contents, sorted by path.
///
/// Read directly rather than asked of git. An earlier version of this test compared
/// `git status --porcelain` before and after, and returned early when the tree was not
/// clean, which meant it did nothing at all whenever anything was staged. A test that
/// silently declines to run is worse than one that fails, because the run reports it as
/// passing.
fn snapshot() -> Vec<(String, Vec<u8>)> {
    fn walk(dir: &std::path::Path, into: &mut Vec<(String, Vec<u8>)>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, into);
            } else if let Ok(bytes) = std::fs::read(&path) {
                into.push((path.to_string_lossy().to_string(), bytes));
            }
        }
    }

    let mut files = Vec::new();
    for tree in ["arm_matrix", "arm_matrix_test/src", "arm_matrix_test/tests"] {
        walk(&std::path::Path::new(ROOT).join(tree), &mut files);
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));
    files
}

#[test]
fn the_generator_survives_its_own_run_and_changes_nothing() {
    let before = snapshot();
    assert!(!before.is_empty(), "the generated trees are missing entirely");

    let report = generate();
    assert!(report.contains("106 cells, 106 assertions"), "unexpected report: {report}");

    let after = snapshot();

    // Per file, so a failure names the one that moved rather than only saying something did.
    let before_paths: Vec<&String> = before.iter().map(|(p, _)| p).collect();
    let after_paths: Vec<&String> = after.iter().map(|(p, _)| p).collect();
    assert_eq!(before_paths, after_paths, "regenerating added or removed files");

    for ((path, was), (_, now)) in before.iter().zip(after.iter()) {
        assert!(was == now, "regenerating changed {path}");
    }

    // Named explicitly, because this is the file that went missing and an empty diff is
    // also what a generator that wrote nothing at all would leave.
    assert!(
        std::path::Path::new(ROOT).join("scripts/generate_arm_matrix.py").exists(),
        "the generator deleted itself",
    );
    assert!(
        std::path::Path::new(ROOT).join("arm_matrix_test/tests").exists(),
        "the hand-written compile-fail suite was removed with the generated trees",
    );
}

#[test]
fn the_generator_refuses_to_run_from_inside_a_tree_it_deletes() {
    // The guard, exercised. Without this the check above passes for as long as nobody moves
    // the script back, and moving it back is exactly what a future tidy-up would do.
    let doomed = std::path::Path::new(ROOT).join("arm_matrix").join("_guard_probe.py");
    let source = std::fs::read_to_string(
        std::path::Path::new(ROOT).join("scripts/generate_arm_matrix.py"),
    )
    .expect("the generator source");

    std::fs::write(&doomed, source).expect("the probe copy");

    let output = Command::new("python3")
        .arg("arm_matrix/_guard_probe.py")
        .current_dir(ROOT)
        .output()
        .expect("python3 runs");

    // Removed before asserting, so a failure does not leave it behind for the test above.
    let _ = std::fs::remove_file(&doomed);

    assert!(!output.status.success(), "the guard let it run from inside arm_matrix/");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("refusing to run"),
        "the refusal says why:\n{}",
        String::from_utf8_lossy(&output.stderr),
    );
}
