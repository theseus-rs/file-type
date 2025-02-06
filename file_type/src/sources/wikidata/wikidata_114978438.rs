use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114978438: FileFormat = FileFormat {
    id: 114_978_438,
    source_type: SourceType::Wikidata,
    name: "StoryView Document",
    extensions: &["syv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
