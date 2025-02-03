use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3641071795: FileFormat = FileFormat {
    id: 3_641_071_795,
    source_type: SourceType::Iana,
    name: "vnd.ibm.secure-container",
    extensions: &[],
    media_types: &["application/vnd.ibm.secure-container"],
    internal_signatures: &[],
    related_formats: &[],
};
