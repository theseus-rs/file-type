use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27474094: FileType = FileType {
    file_format: &FileFormat {
        id: 27_474_094,
        source_type: SourceType::Wikidata,
        name: "BIL/BIP/BSQ Statistics File",
        extensions: &["stx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
