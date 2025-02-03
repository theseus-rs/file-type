use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3301166835: FileFormat = FileFormat {
    id: 3_301_166_835,
    source_type: SourceType::Iana,
    name: "vnd.radiance",
    extensions: &[],
    media_types: &["image/vnd.radiance"],
    internal_signatures: &[],
    related_formats: &[],
};
