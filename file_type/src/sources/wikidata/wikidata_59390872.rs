use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59390872: FileType = FileType {
    file_format: &FileFormat {
        id: 59_390_872,
        source_type: SourceType::Wikidata,
        name: "GraphPad Prism file format",
        extensions: &["pzm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
