use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61641450: FileFormat = FileFormat {
    id: 61_641_450,
    puid: "wikidata/61641450",
    name: "WordPerfect for MS-DOS Document",
    extensions: &["w50", "wp", "wp5", "wpd"],
    media_types: &[
        "application/vnd.wordperfect",
        "application/vnd.wordperfect",
        "application/vnd.wordperfect",
        "application/vnd.wordperfect",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
