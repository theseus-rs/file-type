use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_98713463: FileFormat = FileFormat {
    id: 98_713_463,
    source_type: SourceType::Wikidata,
    name: "POV-Ray",
    extensions: &["pov"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
