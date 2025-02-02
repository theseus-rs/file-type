use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110135368: FileFormat = FileFormat {
    id: 110_135_368,
    source_type: SourceType::Wikidata,
    name: "Serif PhotoPlus Image, version 5-X2",
    extensions: &["spp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
