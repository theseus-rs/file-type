use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126685: FileType = FileType {
    file_format: &FileFormat {
        id: 126_685,
        source_type: SourceType::Wikidata,
        name: "Call Control eXtensible Markup Language",
        extensions: &["ccxml"],
        media_types: &["application/ccxml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
