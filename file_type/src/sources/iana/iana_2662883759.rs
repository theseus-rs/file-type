use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2662883759: FileType = FileType {
    file_format: &FileFormat {
        id: 2_662_883_759,
        source_type: SourceType::Iana,
        name: "vnd.adobe.photoshop",
        extensions: &[],
        media_types: &["image/vnd.adobe.photoshop"],
        signatures: &[],
        related_formats: &[],
    },
};
