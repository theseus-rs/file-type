use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126811621: FileFormat = FileFormat {
    id: 126_811_621,
    source_type: SourceType::Wikidata,
    name: "Bennu bitmap file",
    extensions: &["map"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
