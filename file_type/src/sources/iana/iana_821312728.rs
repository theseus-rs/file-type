use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_821312728: FileFormat = FileFormat {
    id: 821_312_728,
    source_type: SourceType::Iana,
    name: "vnd.cirpack.isdn-ext",
    extensions: &[],
    media_types: &["application/vnd.cirpack.isdn-ext"],
    internal_signatures: &[],
    related_formats: &[],
};
