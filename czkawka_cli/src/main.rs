#![allow(clippy::needless_late_init)]

use std::process;

use clap::Parser;

use commands::Commands;
use czkawka_core::big_file::SearchMode;
#[allow(unused_imports)] // It is used in release for print_results().
use czkawka_core::common_traits::*;
use czkawka_core::similar_images::test_image_conversion_speed;
use czkawka_core::{
    bad_extensions::BadExtensions,
    big_file::{self, BigFile},
    broken_files::{self, BrokenFiles},
    duplicate::DuplicateFinder,
    empty_files::{self, EmptyFiles},
    empty_folder::EmptyFolder,
    invalid_symlinks::{self, InvalidSymlinks},
    same_music::SameMusic,
    similar_images::{return_similarity_from_similarity_preset, SimilarImages},
    similar_videos::SimilarVideos,
    temporary::{self, Temporary},
};

mod commands;

//noinspection ALL
fn main() {
    let command = Commands::from_args();

    #[cfg(debug_assertions)]
    println!("{:?}", command);

    match command {
        Commands::Duplicates {
            directories,
            excluded_directories,
            excluded_items,
            minimal_file_size,
            maximal_file_size,
            minimal_cached_file_size,
            allowed_extensions,
            search_method,
            delete_method,
            hash_type,
            file_to_save,
            not_recursive,
            #[cfg(target_family = "unix")]
            exclude_other_filesystems,
            allow_hard_links,
            dryrun,
            case_sensitive_name_comparison,
        } => {
            let mut df = DuplicateFinder::new();

            df.set_included_directory(directories.directories);
            df.set_excluded_directory(excluded_directories.excluded_directories);
            df.set_excluded_items(excluded_items.excluded_items);
            df.set_minimal_file_size(minimal_file_size);
            df.set_maximal_file_size(maximal_file_size);
            df.set_minimal_cache_file_size(minimal_cached_file_size);
            df.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
            df.set_check_method(search_method);
            df.set_delete_method(delete_method);
            df.set_hash_type(hash_type);
            df.set_recursive_search(!not_recursive.not_recursive);
            #[cfg(target_family = "unix")]
            df.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);
            df.set_ignore_hard_links(!allow_hard_links.allow_hard_links);
            df.set_dryrun(dryrun.dryrun);
            df.set_case_sensitive_name_comparison(case_sensitive_name_comparison.case_sensitive_name_comparison);

            df.find_duplicates(None, None);

            if let Some(file_name) = file_to_save.file_name() {
                if !df.save_results_to_file(file_name) {
                    df.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            df.print_results();
            df.get_text_messages().print_messages();
        }
        Commands::EmptyFolders {
            directories,
            delete_folders,
            file_to_save,
            excluded_directories,
            excluded_items,
            #[cfg(target_family = "unix")]
            exclude_other_filesystems,
        } => {
            let mut ef = EmptyFolder::new();

            ef.set_included_directory(directories.directories);
            ef.set_excluded_directory(excluded_directories.excluded_directories);
            ef.set_excluded_items(excluded_items.excluded_items);
            ef.set_delete_folder(delete_folders);
            #[cfg(target_family = "unix")]
            ef.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);

            ef.find_empty_folders(None, None);

            if let Some(file_name) = file_to_save.file_name() {
                if !ef.save_results_to_file(file_name) {
                    ef.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            ef.print_results();
            ef.get_text_messages().print_messages();
        }
        Commands::BiggestFiles {
            directories,
            excluded_directories,
            excluded_items,
            allowed_extensions,
            number_of_files,
            file_to_save,
            not_recursive,
            #[cfg(target_family = "unix")]
            exclude_other_filesystems,
            delete_files,
            smallest_mode,
        } => {
            let mut bf = BigFile::new();

            bf.set_included_directory(directories.directories);
            bf.set_excluded_directory(excluded_directories.excluded_directories);
            bf.set_excluded_items(excluded_items.excluded_items);
            bf.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
            bf.set_number_of_files_to_check(number_of_files);
            bf.set_recursive_search(!not_recursive.not_recursive);
            #[cfg(target_family = "unix")]
            bf.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);
            if delete_files {
                bf.set_delete_method(big_file::DeleteMethod::Delete);
            }
            if smallest_mode {
                bf.set_search_mode(SearchMode::SmallestFiles);
            }

            bf.find_big_files(None, None);

            if let Some(file_name) = file_to_save.file_name() {
                if !bf.save_results_to_file(file_name) {
                    bf.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            bf.print_results();
            bf.get_text_messages().print_messages();
        }
        Commands::EmptyFiles {
            directories,
            excluded_directories,
            excluded_items,
            allowed_extensions,
            delete_files,
            file_to_save,
            not_recursive,
            #[cfg(target_family = "unix")]
            exclude_other_filesystems,
        } => {
            let mut ef = EmptyFiles::new();

            ef.set_included_directory(directories.directories);
            ef.set_excluded_directory(excluded_directories.excluded_directories);
            ef.set_excluded_items(excluded_items.excluded_items);
            ef.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
            ef.set_recursive_search(!not_recursive.not_recursive);
            #[cfg(target_family = "unix")]
            ef.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);

            if delete_files {
                ef.set_delete_method(empty_files::DeleteMethod::Delete);
            }

            ef.find_empty_files(None, None);

            if let Some(file_name) = file_to_save.file_name() {
                if !ef.save_results_to_file(file_name) {
                    ef.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            ef.print_results();
            ef.get_text_messages().print_messages();
        }
        Commands::Temporary {
            directories,
            excluded_directories,
            excluded_items,
            #[cfg(target_family = "unix")]
            exclude_other_filesystems,
            delete_files,
            file_to_save,
            not_recursive,
        } => {
            let mut tf = Temporary::new();

            tf.set_included_directory(directories.directories);
            tf.set_excluded_directory(excluded_directories.excluded_directories);
            tf.set_excluded_items(excluded_items.excluded_items);
            tf.set_recursive_search(!not_recursive.not_recursive);
            #[cfg(target_family = "unix")]
            tf.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);

            if delete_files {
                tf.set_delete_method(temporary::DeleteMethod::Delete);
            }

            tf.find_temporary_files(None, None);

            if let Some(file_name) = file_to_save.file_name() {
                if !tf.save_results_to_file(file_name) {
                    tf.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            tf.print_results();
            tf.get_text_messages().print_messages();
        }
        Commands::SimilarImages {
            directories,
            excluded_directories,
            excluded_items,
            file_to_save,
            minimal_file_size,
            maximal_file_size,
            similarity_preset,
            not_recursive,
            #[cfg(target_family = "unix")]
            exclude_other_filesystems,
            hash_alg,
            image_filter,
            hash_size,
        } => {
            let mut sf = SimilarImages::new();

            sf.set_included_directory(directories.directories);
            sf.set_excluded_directory(excluded_directories.excluded_directories);
            sf.set_excluded_items(excluded_items.excluded_items);
            sf.set_minimal_file_size(minimal_file_size);
            sf.set_maximal_file_size(maximal_file_size);
            sf.set_recursive_search(!not_recursive.not_recursive);
            #[cfg(target_family = "unix")]
            sf.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);
            sf.set_image_filter(image_filter);
            sf.set_hash_alg(hash_alg);
            sf.set_hash_size(hash_size);

            sf.set_similarity(return_similarity_from_similarity_preset(&similarity_preset, hash_size));

            sf.find_similar_images(None, None);

            if let Some(file_name) = file_to_save.file_name() {
                if !sf.save_results_to_file(file_name) {
                    sf.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            sf.print_results();
            sf.get_text_messages().print_messages();
        }
        Commands::SameMusic {
            directories,
            excluded_directories,
            excluded_items,
            // delete_files,
            file_to_save,
            not_recursive,
            #[cfg(target_family = "unix")]
            exclude_other_filesystems,
            minimal_file_size,
            maximal_file_size,
            music_similarity,
        } => {
            let mut mf = SameMusic::new();

            mf.set_included_directory(directories.directories);
            mf.set_excluded_directory(excluded_directories.excluded_directories);
            mf.set_excluded_items(excluded_items.excluded_items);
            mf.set_minimal_file_size(minimal_file_size);
            mf.set_maximal_file_size(maximal_file_size);
            mf.set_recursive_search(!not_recursive.not_recursive);
            #[cfg(target_family = "unix")]
            mf.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);
            mf.set_music_similarity(music_similarity);

            // if delete_files {
            //     // TODO mf.set_delete_method(same_music::DeleteMethod::Delete);
            // }

            mf.find_same_music(None, None);

            if let Some(file_name) = file_to_save.file_name() {
                if !mf.save_results_to_file(file_name) {
                    mf.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            mf.print_results();
            mf.get_text_messages().print_messages();
        }
        Commands::InvalidSymlinks {
            directories,
            excluded_directories,
            excluded_items,
            allowed_extensions,
            file_to_save,
            not_recursive,
            #[cfg(target_family = "unix")]
            exclude_other_filesystems,
            delete_files,
        } => {
            let mut ifs = InvalidSymlinks::new();

            ifs.set_included_directory(directories.directories);
            ifs.set_excluded_directory(excluded_directories.excluded_directories);
            ifs.set_excluded_items(excluded_items.excluded_items);
            ifs.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
            ifs.set_recursive_search(!not_recursive.not_recursive);
            #[cfg(target_family = "unix")]
            ifs.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);
            if delete_files {
                ifs.set_delete_method(invalid_symlinks::DeleteMethod::Delete);
            }

            ifs.find_invalid_links(None, None);

            if let Some(file_name) = file_to_save.file_name() {
                if !ifs.save_results_to_file(file_name) {
                    ifs.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            ifs.print_results();
            ifs.get_text_messages().print_messages();
        }
        Commands::BrokenFiles {
            directories,
            excluded_directories,
            excluded_items,
            allowed_extensions,
            delete_files,
            file_to_save,
            not_recursive,
            #[cfg(target_family = "unix")]
            exclude_other_filesystems,
        } => {
            let mut br = BrokenFiles::new();

            br.set_included_directory(directories.directories);
            br.set_excluded_directory(excluded_directories.excluded_directories);
            br.set_excluded_items(excluded_items.excluded_items);
            br.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
            br.set_recursive_search(!not_recursive.not_recursive);
            #[cfg(target_family = "unix")]
            br.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);

            if delete_files {
                br.set_delete_method(broken_files::DeleteMethod::Delete);
            }

            br.find_broken_files(None, None);

            if let Some(file_name) = file_to_save.file_name() {
                if !br.save_results_to_file(file_name) {
                    br.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            br.print_results();
            br.get_text_messages().print_messages();
        }
        Commands::SimilarVideos {
            directories,
            excluded_directories,
            excluded_items,
            file_to_save,
            not_recursive,
            #[cfg(target_family = "unix")]
            exclude_other_filesystems,
            tolerance,
            minimal_file_size,
            maximal_file_size,
            allowed_extensions,
        } => {
            let mut vr = SimilarVideos::new();

            vr.set_included_directory(directories.directories);
            vr.set_excluded_directory(excluded_directories.excluded_directories);
            vr.set_excluded_items(excluded_items.excluded_items);
            vr.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
            vr.set_recursive_search(!not_recursive.not_recursive);
            #[cfg(target_family = "unix")]
            vr.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);
            vr.set_minimal_file_size(minimal_file_size);
            vr.set_maximal_file_size(maximal_file_size);
            vr.set_tolerance(tolerance);

            vr.find_similar_videos(None, None);

            if let Some(file_name) = file_to_save.file_name() {
                if !vr.save_results_to_file(file_name) {
                    vr.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            vr.print_results();
            vr.get_text_messages().print_messages();
        }
        Commands::BadExtensions {
            directories,
            excluded_directories,
            excluded_items,
            file_to_save,
            not_recursive,
            #[cfg(target_family = "unix")]
            exclude_other_filesystems,
            allowed_extensions,
        } => {
            let mut be = BadExtensions::new();

            be.set_included_directory(directories.directories);
            be.set_excluded_directory(excluded_directories.excluded_directories);
            be.set_excluded_items(excluded_items.excluded_items);
            be.set_allowed_extensions(allowed_extensions.allowed_extensions.join(","));
            be.set_recursive_search(!not_recursive.not_recursive);
            #[cfg(target_family = "unix")]
            be.set_exclude_other_filesystems(exclude_other_filesystems.exclude_other_filesystems);

            if let Some(file_name) = file_to_save.file_name() {
                if !be.save_results_to_file(file_name) {
                    be.get_text_messages().print_messages();
                    process::exit(1);
                }
            }

            be.find_bad_extensions_files(None, None);

            #[cfg(not(debug_assertions))] // This will show too much probably unnecessary data to debug, comment line only if needed
            be.print_results();
            be.get_text_messages().print_messages();
        }
        Commands::Tester {} => {
            test_image_conversion_speed();
        }
    }
}
