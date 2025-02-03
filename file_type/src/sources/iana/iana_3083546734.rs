use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3083546734: FileFormat = FileFormat {
    id: 3_083_546_734,
    source_type: SourceType::Iana,
    name: "dicom+xml",
    extensions: &[],
    media_types: &["application/dicom+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
