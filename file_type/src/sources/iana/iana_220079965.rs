use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_220079965: FileType = FileType {
    file_format: &FileFormat {
        id: 220_079_965,
        source_type: SourceType::Iana,
        name: "vnd.ms-wmdrm.meter-resp",
        extensions: &[],
        media_types: &["application/vnd.ms-wmdrm.meter-resp"],
        signatures: &[],
        related_formats: &[],
    },
};
