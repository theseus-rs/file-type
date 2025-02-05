use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1162286966: FileFormat = FileFormat {
    id: 1_162_286_966,
    source_type: SourceType::Iana,
    name: "vnd.geogebra.pinboard",
    extensions: &[],
    media_types: &["application/vnd.geogebra.pinboard"],
    signatures: &[],
    related_formats: &[],
};
