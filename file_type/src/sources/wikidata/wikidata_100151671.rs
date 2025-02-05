use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100151671: FileFormat = FileFormat {
    id: 100_151_671,
    source_type: SourceType::Wikidata,
    name: "Bruker PDZ",
    extensions: &["pdz", "xpdz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
