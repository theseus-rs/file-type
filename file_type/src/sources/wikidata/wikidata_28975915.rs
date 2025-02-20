use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28975915: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_915,
        source_type: SourceType::Wikidata,
        name: "ZPR",
        extensions: &["zpr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
