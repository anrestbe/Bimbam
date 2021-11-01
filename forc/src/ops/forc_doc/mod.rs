mod html;

use crate::{
    cli::{BuildCommand, DocCommand},
    utils::{
        cli_error::CliError,
        helpers::{find_manifest_dir, get_sway_files},
    },
};

use super::forc_build;

pub fn doc(_command: DocCommand) -> Result<(), CliError> {
    let build_command = BuildCommand {
        path: None,
        print_finalized_asm: false,
        print_intermediate_asm: false,
        binary_outfile: None,
        offline_mode: false,
        silent_mode: false,
    };

    match forc_build::build(build_command) {
        Ok(_) => {
            let curr_dir = std::env::current_dir()?;

            match find_manifest_dir(&curr_dir) {
                Some(path_buf) => {
                    let files = get_sway_files(path_buf);

                    for file in files {
                        if let Ok(file_content) = std::fs::read_to_string(&file) {
                            let parsed_res = core_lang::parse(&file_content, None);
                            if let Some(value) = parsed_res.value {
                                html::build_from_tree(value)?;
                            }
                        }
                    }

                    Ok(())
                }
                None => Err(CliError::manifest_file_missing(curr_dir)),
            }
        }
        Err(err) => Err(err.into()),
    }
}
