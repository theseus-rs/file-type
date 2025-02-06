use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116790056: FileFormat = FileFormat {
    id: 116_790_056,
    source_type: SourceType::Wikidata,
    name: "WordPad Document",
    extensions: &["doc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
