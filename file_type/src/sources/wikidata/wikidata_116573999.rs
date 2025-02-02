use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116573999: FileFormat = FileFormat {
    id: 116_573_999,
    source_type: SourceType::Wikidata,
    name: "CoffeeCup Google Site Mapper Profile",
    extensions: &["csm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
