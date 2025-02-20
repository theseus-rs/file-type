use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130618874: FileType = FileType {
    file_format: &FileFormat {
        id: 130_618_874,
        source_type: SourceType::Wikidata,
        name: "Redcode file format",
        extensions: &["cw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
