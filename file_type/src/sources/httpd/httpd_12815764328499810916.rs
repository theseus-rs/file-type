use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12815764328499810916: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "cmu raster",
    extensions: &["ras"],
    media_types: &["image/x-cmu-raster"],
    internal_signatures: &[],
    related_formats: &[],
};
