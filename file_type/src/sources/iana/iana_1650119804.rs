use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1650119804: FileType = FileType {
    file_format: &FileFormat {
        id: 1_650_119_804,
        source_type: SourceType::Iana,
        name: "vnd.ims.imsccv1p1",
        extensions: &[],
        media_types: &["application/vnd.ims.imsccv1p1"],
        signatures: &[],
        related_formats: &[],
    },
};
