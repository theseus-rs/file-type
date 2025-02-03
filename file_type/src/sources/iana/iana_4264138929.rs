use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4264138929: FileFormat = FileFormat {
    id: 4_264_138_929,
    source_type: SourceType::Iana,
    name: "pwg-raster",
    extensions: &[],
    media_types: &["image/pwg-raster"],
    internal_signatures: &[],
    related_formats: &[],
};
