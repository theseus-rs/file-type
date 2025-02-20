use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3922324027: FileType = FileType {
    file_format: &FileFormat {
        id: 3_922_324_027,
        source_type: SourceType::Iana,
        name: "VDVI",
        extensions: &[],
        media_types: &["audio/VDVI"],
        signatures: &[],
        related_formats: &[],
    },
};
