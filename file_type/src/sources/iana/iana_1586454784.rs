use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1586454784: FileType = FileType {
    file_format: &FileFormat {
        id: 1_586_454_784,
        source_type: SourceType::Iana,
        name: "vnd.vmx.cvsd",
        extensions: &[],
        media_types: &["audio/vnd.vmx.cvsd"],
        signatures: &[],
        related_formats: &[],
    },
};
