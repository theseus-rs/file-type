use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110039764: FileFormat = FileFormat {
    id: 110_039_764,
    source_type: SourceType::Wikidata,
    name: "Timeline Maker Document",
    extensions: &["tlm", "tlm3", "tlm4", "tlmp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
