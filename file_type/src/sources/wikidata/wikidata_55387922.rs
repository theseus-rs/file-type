use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_55387922: FileType = FileType {
    file_format: &FileFormat {
        id: 55_387_922,
        source_type: SourceType::Wikidata,
        name: "Visual Molecular Dynamics file format",
        extensions: &["vmd"],
        media_types: &["chemical/x-vmd"],
        signatures: &[],
        related_formats: &[],
    },
};
