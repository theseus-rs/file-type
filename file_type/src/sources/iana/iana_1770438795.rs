use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1770438795: FileFormat = FileFormat {
    id: 1_770_438_795,
    source_type: SourceType::Iana,
    name: "vnd.fujifilm.fb.docuworks",
    extensions: &[],
    media_types: &["application/vnd.fujifilm.fb.docuworks"],
    internal_signatures: &[],
    related_formats: &[],
};
