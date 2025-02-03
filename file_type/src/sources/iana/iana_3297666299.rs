use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3297666299: FileFormat = FileFormat {
    id: 3_297_666_299,
    source_type: SourceType::Iana,
    name: "vnd.afpc.modca-formdef",
    extensions: &[],
    media_types: &["application/vnd.afpc.modca-formdef"],
    internal_signatures: &[],
    related_formats: &[],
};
