use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_62664735: FileFormat = FileFormat {
    id: 62_664_735,
    source_type: SourceType::Wikidata,
    name: "Wordperfect Secondary File, version 5.1 and 5.2",
    extensions: &["doc"],
    media_types: &["application/wordperfect5.1"],
    signatures: &[],
    related_formats: &[],
};
