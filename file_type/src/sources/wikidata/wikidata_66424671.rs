use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66424671: FileFormat = FileFormat {
    id: 66_424_671,
    source_type: SourceType::Wikidata,
    name: "WordPerfect macro file format",
    extensions: &["wcm", "wpm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
