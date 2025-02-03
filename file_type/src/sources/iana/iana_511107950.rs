use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_511107950: FileFormat = FileFormat {
    id: 511_107_950,
    source_type: SourceType::Iana,
    name: "vnd.nearst.inv+json",
    extensions: &[],
    media_types: &["application/vnd.nearst.inv+json"],
    internal_signatures: &[],
    related_formats: &[],
};
