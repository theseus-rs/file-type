use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_98713463: FileFormat = FileFormat {
    id: 98_713_463,
    source_type: SourceType::Wikidata,
    name: "POV-Ray",
    extensions: &["pov"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
