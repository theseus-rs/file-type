use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_131144297: FileFormat = FileFormat {
    id: 131_144_297,
    source_type: SourceType::Wikidata,
    name: "Solidity source code file",
    extensions: &["sol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
