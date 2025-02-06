use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125936447: FileFormat = FileFormat {
    id: 125_936_447,
    source_type: SourceType::Wikidata,
    name: "Atrac Codec File v.1",
    extensions: &["aea"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
