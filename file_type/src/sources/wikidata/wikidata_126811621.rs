use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_126811621: FileFormat = FileFormat {
    id: 126_811_621,
    source_type: SourceType::Wikidata,
    name: "Bennu bitmap file",
    extensions: &["map"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
