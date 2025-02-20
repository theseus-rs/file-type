use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4192310636: FileType = FileType {
    file_format: &FileFormat {
        id: 4_192_310_636,
        source_type: SourceType::Iana,
        name: "vnd.shx",
        extensions: &[],
        media_types: &["application/vnd.shx"],
        signatures: &[],
        related_formats: &[],
    },
};
