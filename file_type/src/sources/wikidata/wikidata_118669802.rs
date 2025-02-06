use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_118669802: FileFormat = FileFormat {
    id: 118_669_802,
    source_type: SourceType::Wikidata,
    name: "Shade To Manga Studio file",
    extensions: &["stc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
