use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3976978134: FileFormat = FileFormat {
    id: 3_976_978_134,
    source_type: SourceType::Iana,
    name: "vnd.oma.cab-pcc+xml",
    extensions: &[],
    media_types: &["application/vnd.oma.cab-pcc+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
