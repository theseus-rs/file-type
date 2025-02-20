use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_523598520: FileType = FileType {
    file_format: &FileFormat {
        id: 523_598_520,
        source_type: SourceType::Iana,
        name: "vnd.intertrust.digibox",
        extensions: &[],
        media_types: &["application/vnd.intertrust.digibox"],
        signatures: &[],
        related_formats: &[],
    },
};
