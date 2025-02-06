use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2823079662: FileFormat = FileFormat {
    id: 2_823_079_662,
    source_type: SourceType::Iana,
    name: "grib",
    extensions: &[],
    media_types: &["application/grib"],
    signatures: &[],
    related_formats: &[],
};
