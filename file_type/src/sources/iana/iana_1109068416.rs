use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1109068416: FileType = FileType {
    file_format: &FileFormat {
        id: 1_109_068_416,
        source_type: SourceType::Iana,
        name: "dicom-rle",
        extensions: &[],
        media_types: &["image/dicom-rle"],
        signatures: &[],
        related_formats: &[],
    },
};
