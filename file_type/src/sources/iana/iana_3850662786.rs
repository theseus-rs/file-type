use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3850662786: FileFormat = FileFormat {
    id: 3_850_662_786,
    source_type: SourceType::Iana,
    name: "vnd.netfpx",
    extensions: &[],
    media_types: &["application/vnd.netfpx"],
    signatures: &[],
    related_formats: &[],
};
