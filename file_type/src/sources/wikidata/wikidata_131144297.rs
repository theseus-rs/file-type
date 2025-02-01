use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131144297: FileFormat = FileFormat {
    id: 131_144_297,
    puid: "wikidata/131144297",
    name: "Solidity source code file",
    extensions: &["sol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
