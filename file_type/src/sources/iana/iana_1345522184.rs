use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1345522184: FileType = FileType {
    file_format: &FileFormat {
        id: 1_345_522_184,
        source_type: SourceType::Iana,
        name: "ppsp-tracker+json",
        extensions: &[],
        media_types: &["application/ppsp-tracker+json"],
        signatures: &[],
        related_formats: &[],
    },
};
