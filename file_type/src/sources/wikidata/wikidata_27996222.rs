use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27996222: FileFormat = FileFormat {
    id: 27_996_222,
    source_type: SourceType::Wikidata,
    name: "Fallout character description",
    extensions: &["gcd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
