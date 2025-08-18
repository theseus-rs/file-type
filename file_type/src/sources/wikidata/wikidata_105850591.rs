use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105850591: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_591,
        source_type: SourceType::Wikidata,
        name: "CryEngine Project (generic)",
        extensions: &["cryproject"],
        media_types: &["application/json"],
        signatures: &[],
        related_formats: &[],
    },
};
