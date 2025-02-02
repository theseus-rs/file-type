use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112944076: FileFormat = FileFormat {
    id: 112_944_076,
    source_type: SourceType::Wikidata,
    name: "GameExchange2 lights file",
    extensions: &["GLF"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
