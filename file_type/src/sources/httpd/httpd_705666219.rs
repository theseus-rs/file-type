use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_705666219: FileFormat = FileFormat {
    id: 705_666_219,
    source_type: SourceType::Httpd,
    name: "midi",
    extensions: &["mid", "midi", "kar", "rmi"],
    media_types: &["audio/midi"],
    internal_signatures: &[],
    related_formats: &[],
};
