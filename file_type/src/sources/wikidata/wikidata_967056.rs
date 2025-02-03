use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_967056: FileFormat = FileFormat {
    id: 967_056,
    source_type: SourceType::Wikidata,
    name: "SoundFont",
    extensions: &["sf2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
