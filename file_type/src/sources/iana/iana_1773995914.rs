use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1773995914: FileType = FileType {
    file_format: &FileFormat {
        id: 1_773_995_914,
        source_type: SourceType::Iana,
        name: "sensml-exi",
        extensions: &[],
        media_types: &["application/sensml-exi"],
        signatures: &[],
        related_formats: &[],
    },
};
