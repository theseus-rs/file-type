use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116790056: FileFormat = FileFormat {
    id: 116_790_056,
    source_type: SourceType::Wikidata,
    name: "WordPad Document",
    extensions: &["doc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
