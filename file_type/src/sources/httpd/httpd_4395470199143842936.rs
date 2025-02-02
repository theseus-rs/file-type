use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4395470199143842936: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "matroska",
    extensions: &["mkv", "mk3d", "mks"],
    media_types: &["video/x-matroska"],
    internal_signatures: &[],
    related_formats: &[],
};
