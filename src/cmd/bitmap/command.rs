// Copyright 2020 Revcore Technologies Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{Error, ErrorKind};
use proj::{AloeVeraProject, Jsonable};

use crate::cmd::common::{self, GlobalArgs};
use vera::VeraBitmap;

/// Arguments for bitmap
pub struct BitmapInitArgs {
	pub id: String,
	pub imageset_id: String,
}

/// Bitmap init command
pub fn bitmap_init(g_args: &GlobalArgs, args: &BitmapInitArgs) -> Result<(), Error> {
	// load up the project json
	let project_file = match &g_args.project_file {
		Some(f) => f,
		None => {
			return Err(ErrorKind::ArgumentError("Missing project file name".to_string()).into())
		}
	};
	info!("Adding bitmap into project: {}", project_file);
	let proj_json = common::read_file_string(&project_file)?;
	let mut proj = AloeVeraProject::new_from_json(&proj_json)?;
	let imageset = match proj.imagesets.get(&args.imageset_id) {
		Some(i) => i,
		None => {
			let msg = format!(
				"Imageset with id {} does not exist in project file.",
				args.imageset_id
			);
			return Err(ErrorKind::ArgumentError(msg).into());
		}
	};
	let bitmap = VeraBitmap::init_from_imageset(&args.id, &imageset)?;
	proj.bitmaps.insert(args.id.clone(), bitmap);
	common::output_to_file(&project_file, &proj.to_json()?)?;

	Ok(())
}
