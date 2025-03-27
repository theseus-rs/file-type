use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_197972: FileType = FileType {
    file_format: &FileFormat {
        id: 197_972,
        source_type: SourceType::Wikidata,
        name: "AMV video format",
        extensions: &["amv", "mtv"],
        media_types: &["video/x-amv"],
        signatures: &[],
        related_formats: &[],
    },
};
