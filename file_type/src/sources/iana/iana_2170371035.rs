use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2170371035: FileFormat = FileFormat {
    id: 2_170_371_035,
    source_type: SourceType::Iana,
    name: "vnd.dolby.pl2x",
    extensions: &[],
    media_types: &["audio/vnd.dolby.pl2x"],
    internal_signatures: &[],
    related_formats: &[],
};
