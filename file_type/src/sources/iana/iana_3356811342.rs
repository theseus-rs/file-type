use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3356811342: FileFormat = FileFormat {
    id: 3_356_811_342,
    source_type: SourceType::Iana,
    name: "dicom",
    extensions: &[],
    media_types: &["application/dicom"],
    signatures: &[],
    related_formats: &[],
};
