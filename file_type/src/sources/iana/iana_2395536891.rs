use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2395536891: FileFormat = FileFormat {
    id: 2_395_536_891,
    source_type: SourceType::Iana,
    name: "vnd.ms-ims",
    extensions: &[],
    media_types: &["application/vnd.ms-ims"],
    internal_signatures: &[],
    related_formats: &[],
};
