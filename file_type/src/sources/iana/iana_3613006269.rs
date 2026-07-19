use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3613006269: FileType = FileType {
    file_format: &FileFormat {
        id: 3_613_006_269,
        source_type: SourceType::Iana,
        name: "aas+zip",
        extensions: &[],
        media_types: &["application/aas+zip"],
        signatures: &[],
        related_formats: &[],
    },
};
