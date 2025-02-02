use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100377205: FileFormat = FileFormat {
    id: 100_377_205,
    source_type: SourceType::Wikidata,
    name: "WordPerfect 4.2 Encrypted Document",
    extensions: &["wp"],
    media_types: &["application/vnd.wordperfect"],
    internal_signatures: &[],
    related_formats: &[],
};
