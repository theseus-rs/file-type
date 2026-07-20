use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2214143539: FileType = FileType {
    file_format: &FileFormat {
        id: 2_214_143_539,
        source_type: SourceType::Iana,
        name: "vnd.majikah.bundle",
        extensions: &[],
        media_types: &["application/vnd.majikah.bundle"],
        signatures: &[],
        related_formats: &[],
    },
};
