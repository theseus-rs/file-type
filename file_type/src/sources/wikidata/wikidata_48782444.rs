use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48782444: FileType = FileType {
    file_format: &FileFormat {
        id: 48_782_444,
        source_type: SourceType::Wikidata,
        name: "ACBM Graphics",
        extensions: &["acb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
