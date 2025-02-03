use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_340917823: FileFormat = FileFormat {
    id: 340_917_823,
    source_type: SourceType::Iana,
    name: "vnd.oma.push",
    extensions: &[],
    media_types: &["application/vnd.oma.push"],
    internal_signatures: &[],
    related_formats: &[],
};
