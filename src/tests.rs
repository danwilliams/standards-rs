//		Packages

use crate::*;
use std::{
	fs::File,
	io::prelude::*,
	path::PathBuf,
};
use tempfile::{TempDir, self};



//		Functions

//		setup_testfile															
pub fn setup_testfile() -> (TempDir, PathBuf, File) {
	//	Set up testing environment and check that it works
	let tempdir  = tempfile::tempdir().expect("Failed to create temporary directory");
	let filepath = tempdir.path().join("test.txt");
	let file     = File::create(&filepath).expect("Failed to create test file");
	(tempdir, filepath, file)
}


