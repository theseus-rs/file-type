use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600401: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_401,
        source_type: SourceType::Wikidata,
        name: "Assembly manifest (Windows)",
        extensions: &["manifest"],
        media_types: &["application/x-ms-manifest"],
        signatures: &[],
        related_formats: &[],
    },
};
