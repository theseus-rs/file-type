use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_713073804: FileType = FileType {
    file_format: &FileFormat {
        id: 713_073_804,
        source_type: SourceType::Iana,
        name: "hl7v2",
        extensions: &[],
        media_types: &["text/hl7v2"],
        signatures: &[],
        related_formats: &[],
    },
};
