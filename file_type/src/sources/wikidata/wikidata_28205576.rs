use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205576: FileFormat = FileFormat {
    id: 28_205_576,
    source_type: SourceType::Wikidata,
    name: "OLPC 565",
    extensions: &["565"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x35, 0x36, 0x35])],
            },
        }],
    }],
    related_formats: &[],
};
