use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6913682602939238681: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "midi",
    extensions: &["mid", "midi", "kar", "rmi"],
    media_types: &["audio/midi"],
    internal_signatures: &[],
    related_formats: &[],
};
