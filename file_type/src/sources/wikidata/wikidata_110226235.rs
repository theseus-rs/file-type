use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110226235: FileFormat = FileFormat {
    id: 110_226_235,
    source_type: SourceType::Wikidata,
    name: "NeoDesk Icon File, version 3",
    extensions: &["nic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
