use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_623334: FileType = FileType {
    file_format: &FileFormat {
        id: 623_334,
        source_type: SourceType::Wikidata,
        name: ".exe",
        extensions: &["exe"],
        media_types: &["application/vnd.microsoft.portable-executable"],
        signatures: &[],
        related_formats: &[],
    },
};
