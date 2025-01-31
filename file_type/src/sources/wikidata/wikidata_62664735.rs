use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_62664735: FileFormat = FileFormat {
    id: 62_664_735,
    puid: "wikidata/62664735",
    name: "Wordperfect Secondary File, version 5.1 and 5.2",
    extensions: &["doc"],
    media_types: &["application/wordperfect5.1"],
    internal_signatures: &[],
    related_formats: &[],
};
