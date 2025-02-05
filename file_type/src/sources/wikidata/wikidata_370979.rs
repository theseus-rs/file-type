use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_370979: FileFormat = FileFormat {
    id: 370_979,
    source_type: SourceType::Wikidata,
    name: "Amigaguide",
    extensions: &["guide"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
