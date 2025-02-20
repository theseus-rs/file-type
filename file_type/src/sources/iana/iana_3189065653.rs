use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3189065653: FileType = FileType {
    file_format: &FileFormat {
        id: 3_189_065_653,
        source_type: SourceType::Iana,
        name: "vnd.uplanet.list",
        extensions: &[],
        media_types: &["application/vnd.uplanet.list"],
        signatures: &[],
        related_formats: &[],
    },
};
