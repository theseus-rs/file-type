use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3507283135: FileFormat = FileFormat {
    id: 3_507_283_135,
    source_type: SourceType::Iana,
    name: "vnd.afpc.modca-pagesegment",
    extensions: &[],
    media_types: &["application/vnd.afpc.modca-pagesegment"],
    internal_signatures: &[],
    related_formats: &[],
};
