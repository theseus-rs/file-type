use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27996239: FileFormat = FileFormat {
    id: 27_996_239,
    source_type: SourceType::Wikidata,
    name: "Faster than Light saved game",
    extensions: &["sav"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
