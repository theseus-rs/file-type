use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116573999: FileFormat = FileFormat {
    id: 116_573_999,
    puid: "wikidata/116573999",
    name: "CoffeeCup Google Site Mapper Profile",
    extensions: &["csm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
