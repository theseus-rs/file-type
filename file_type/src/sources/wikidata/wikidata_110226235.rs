use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110226235: FileFormat = FileFormat {
    id: 110_226_235,
    source_type: SourceType::Wikidata,
    name: "NeoDesk Icon File, version 3",
    extensions: &["nic"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
