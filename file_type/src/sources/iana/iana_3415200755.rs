use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3415200755: FileFormat = FileFormat {
    id: 3_415_200_755,
    source_type: SourceType::Iana,
    name: "vnd.mapbox-vector-tile",
    extensions: &[],
    media_types: &["application/vnd.mapbox-vector-tile"],
    internal_signatures: &[],
    related_formats: &[],
};
