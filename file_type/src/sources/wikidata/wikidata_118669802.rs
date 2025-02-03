use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_118669802: FileFormat = FileFormat {
    id: 118_669_802,
    source_type: SourceType::Wikidata,
    name: "Shade To Manga Studio file",
    extensions: &["stc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
