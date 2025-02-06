use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116419544: FileFormat = FileFormat {
    id: 116_419_544,
    source_type: SourceType::Wikidata,
    name: "CoffeeCup Website Color Schemer file",
    extensions: &["ccs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
