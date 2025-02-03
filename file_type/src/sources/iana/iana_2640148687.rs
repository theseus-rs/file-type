use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2640148687: FileFormat = FileFormat {
    id: 2_640_148_687,
    source_type: SourceType::Iana,
    name: "vnd.filmit.zfc",
    extensions: &[],
    media_types: &["application/vnd.filmit.zfc"],
    internal_signatures: &[],
    related_formats: &[],
};
