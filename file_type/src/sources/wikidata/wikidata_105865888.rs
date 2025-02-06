use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865888: FileFormat = FileFormat {
    id: 105_865_888,
    source_type: SourceType::Wikidata,
    name: "Gerber Photoplot",
    extensions: &["pho"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
