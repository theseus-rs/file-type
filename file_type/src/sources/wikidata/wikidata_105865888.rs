use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865888: FileFormat = FileFormat {
    id: 105_865_888,
    source_type: SourceType::Wikidata,
    name: "Gerber Photoplot",
    extensions: &["pho"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
