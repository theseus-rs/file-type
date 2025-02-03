use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9632120318872693343: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "dece audio",
    extensions: &["uva", "uvva"],
    media_types: &["audio/vnd.dece.audio"],
    internal_signatures: &[],
    related_formats: &[],
};
