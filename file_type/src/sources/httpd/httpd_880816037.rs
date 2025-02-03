use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_880816037: FileFormat = FileFormat {
    id: 880_816_037,
    source_type: SourceType::Httpd,
    name: "cmu raster",
    extensions: &["ras"],
    media_types: &["image/x-cmu-raster"],
    internal_signatures: &[],
    related_formats: &[],
};
