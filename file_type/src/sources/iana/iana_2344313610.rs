use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2344313610: FileFormat = FileFormat {
    id: 2_344_313_610,
    source_type: SourceType::Iana,
    name: "vnd.ciedi",
    extensions: &[],
    media_types: &["application/vnd.ciedi"],
    internal_signatures: &[],
    related_formats: &[],
};
