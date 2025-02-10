use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66142150: FileType = FileType {
    file_format: &FileFormat {
        id: 66_142_150,
        source_type: SourceType::Wikidata,
        name: "ADE file format",
        extensions: &["ade"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
