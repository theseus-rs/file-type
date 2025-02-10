use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4196913877: FileType = FileType {
    file_format: &FileFormat {
        id: 4_196_913_877,
        source_type: SourceType::Iana,
        name: "vnd.osgi.bundle",
        extensions: &[],
        media_types: &["application/vnd.osgi.bundle"],
        signatures: &[],
        related_formats: &[],
    },
};
