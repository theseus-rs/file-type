use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3214346991: FileFormat = FileFormat {
    id: 3_214_346_991,
    source_type: SourceType::Iana,
    name: "vnd.fujifilm.fb.docuworks.container",
    extensions: &[],
    media_types: &["application/vnd.fujifilm.fb.docuworks.container"],
    signatures: &[],
    related_formats: &[],
};
