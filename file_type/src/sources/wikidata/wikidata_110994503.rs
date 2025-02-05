use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110994503: FileFormat = FileFormat {
    id: 110_994_503,
    source_type: SourceType::Wikidata,
    name: "Micrografx Pictures Publisher",
    extensions: &["pdf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
