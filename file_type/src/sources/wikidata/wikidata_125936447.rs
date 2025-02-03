use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125936447: FileFormat = FileFormat {
    id: 125_936_447,
    source_type: SourceType::Wikidata,
    name: "Atrac Codec File v.1",
    extensions: &["aea"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
