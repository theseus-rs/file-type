use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_11414351502892999160: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "ms playready media pya",
    extensions: &["pya"],
    media_types: &["audio/vnd.ms-playready.media.pya"],
    internal_signatures: &[],
    related_formats: &[],
};
