use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131471383: FileType = FileType {
    file_format: &FileFormat {
        id: 131_471_383,
        source_type: SourceType::Wikidata,
        name: "Compressed MGH file format",
        extensions: &["mgh.gz", "mgz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
