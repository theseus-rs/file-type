use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130974131: FileType = FileType {
    file_format: &FileFormat {
        id: 130_974_131,
        source_type: SourceType::Wikidata,
        name: "Silver source code file",
        extensions: &["sil", "ver"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
