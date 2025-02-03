use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110994503: FileFormat = FileFormat {
    id: 110_994_503,
    source_type: SourceType::Wikidata,
    name: "Micrografx Pictures Publisher",
    extensions: &["pdf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
