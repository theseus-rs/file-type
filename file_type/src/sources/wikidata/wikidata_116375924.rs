use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116375924: FileType = FileType {
    file_format: &FileFormat {
        id: 116_375_924,
        source_type: SourceType::Wikidata,
        name: "Access Database (2003 and earlier)",
        extensions: &["mdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
