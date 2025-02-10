use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_370260795: FileType = FileType {
    file_format: &FileFormat {
        id: 370_260_795,
        source_type: SourceType::Iana,
        name: "vnd.oai.workflows",
        extensions: &[],
        media_types: &["application/vnd.oai.workflows"],
        signatures: &[],
        related_formats: &[],
    },
};
