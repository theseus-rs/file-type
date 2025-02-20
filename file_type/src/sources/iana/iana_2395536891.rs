use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2395536891: FileType = FileType {
    file_format: &FileFormat {
        id: 2_395_536_891,
        source_type: SourceType::Iana,
        name: "vnd.ms-ims",
        extensions: &[],
        media_types: &["application/vnd.ms-ims"],
        signatures: &[],
        related_formats: &[],
    },
};
