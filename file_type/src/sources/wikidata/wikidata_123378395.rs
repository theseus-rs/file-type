use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123378395: FileFormat = FileFormat {
    id: 123_378_395,
    source_type: SourceType::Wikidata,
    name: "Radiosity Solution file",
    extensions: &["lwr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
