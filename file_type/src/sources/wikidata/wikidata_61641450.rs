use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61641450: FileFormat = FileFormat {
    id: 61_641_450,
    source_type: SourceType::Wikidata,
    name: "WordPerfect for MS-DOS Document",
    extensions: &["w50", "wp", "wp5", "wpd"],
    media_types: &["application/vnd.wordperfect"],
    signatures: &[],
    related_formats: &[],
};
