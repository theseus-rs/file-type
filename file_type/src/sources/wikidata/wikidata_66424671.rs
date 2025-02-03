use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66424671: FileFormat = FileFormat {
    id: 66_424_671,
    source_type: SourceType::Wikidata,
    name: "WordPerfect macro file format",
    extensions: &["wcm", "wpm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
