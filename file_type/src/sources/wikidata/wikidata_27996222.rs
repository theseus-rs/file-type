use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27996222: FileFormat = FileFormat {
    id: 27_996_222,
    source_type: SourceType::Wikidata,
    name: "Fallout character description",
    extensions: &["gcd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
