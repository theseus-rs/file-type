use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_21040945: FileFormat = FileFormat {
    id: 21_040_945,
    source_type: SourceType::Wikidata,
    name: "Digitrakker format",
    extensions: &["mdl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
