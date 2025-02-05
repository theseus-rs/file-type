use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_132145897: FileFormat = FileFormat {
    id: 132_145_897,
    source_type: SourceType::Wikidata,
    name: "Braille Ready Format",
    extensions: &["brf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
