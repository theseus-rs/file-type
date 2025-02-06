use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131144297: FileFormat = FileFormat {
    id: 131_144_297,
    source_type: SourceType::Wikidata,
    name: "Solidity source code file",
    extensions: &["sol"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
