use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_777561: FileType = FileType {
    file_format: &FileFormat {
        id: 777_561,
        source_type: SourceType::Wikidata,
        name: "BinHex",
        extensions: &["hcx", "hex", "hqx"],
        media_types: &[
            "application/binhex",
            "application/mac-binhex",
            "application/mac-binhex40",
        ],
        signatures: &[],
        related_formats: &[],
    },
};
