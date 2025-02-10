use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28600470: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_470,
        source_type: SourceType::Wikidata,
        name: "DER encoded RSA private key",
        extensions: &["key"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
