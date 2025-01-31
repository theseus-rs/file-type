use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116419544: FileFormat = FileFormat {
    id: 116_419_544,
    puid: "wikidata/116419544",
    name: "CoffeeCup Website Color Schemer file",
    extensions: &["ccs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
