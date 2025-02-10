use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28757953: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_953,
        source_type: SourceType::Wikidata,
        name: "HGT",
        extensions: &["hgt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
