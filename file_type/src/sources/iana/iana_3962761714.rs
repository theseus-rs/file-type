use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3962761714: FileFormat = FileFormat {
    id: 3_962_761_714,
    source_type: SourceType::Iana,
    name: "vnd.osgeo.mapguide.package",
    extensions: &[],
    media_types: &["application/vnd.osgeo.mapguide.package"],
    signatures: &[],
    related_formats: &[],
};
