use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110039764: FileFormat = FileFormat {
    id: 110_039_764,
    source_type: SourceType::Wikidata,
    name: "Timeline Maker Document",
    extensions: &["tlm", "tlm3", "tlm4", "tlmp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
