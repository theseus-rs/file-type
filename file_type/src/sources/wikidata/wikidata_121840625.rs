use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121840625: FileFormat = FileFormat {
    id: 121_840_625,
    source_type: SourceType::Wikidata,
    name: "Common Instrument File 1",
    extensions: &["ci1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
