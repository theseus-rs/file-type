use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1109068416: FileFormat = FileFormat {
    id: 1_109_068_416,
    source_type: SourceType::Iana,
    name: "dicom-rle",
    extensions: &[],
    media_types: &["image/dicom-rle"],
    internal_signatures: &[],
    related_formats: &[],
};
