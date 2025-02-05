use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_132145897: FileFormat = FileFormat {
    id: 132_145_897,
    source_type: SourceType::Wikidata,
    name: "Braille Ready Format",
    extensions: &["brf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
