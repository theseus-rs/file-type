use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_370979: FileFormat = FileFormat {
    id: 370_979,
    source_type: SourceType::Wikidata,
    name: "Amigaguide",
    extensions: &["guide"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
