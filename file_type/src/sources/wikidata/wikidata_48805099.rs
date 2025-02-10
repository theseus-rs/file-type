use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48805099: FileType = FileType {
    file_format: &FileFormat {
        id: 48_805_099,
        source_type: SourceType::Wikidata,
        name: "Btrieve Database",
        extensions: &["btr"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
