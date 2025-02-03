use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66309235: FileFormat = FileFormat {
    id: 66_309_235,
    source_type: SourceType::Wikidata,
    name: "Access Blank Project Template",
    extensions: &["adn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
