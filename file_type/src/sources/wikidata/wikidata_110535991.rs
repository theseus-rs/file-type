use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110535991: FileFormat = FileFormat {
    id: 110_535_991,
    source_type: SourceType::Wikidata,
    name: "Movie Magic Screenwriter backup document",
    extensions: &["bk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
