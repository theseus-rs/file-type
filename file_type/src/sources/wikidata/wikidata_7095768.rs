use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7095768: FileType = FileType {
    file_format: &FileFormat {
        id: 7_095_768,
        source_type: SourceType::Wikidata,
        name: "OpenDRIVE",
        extensions: &["xodr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
