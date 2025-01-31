use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205576: FileFormat = FileFormat {
    id: 28_205_576,
    puid: "wikidata/28205576",
    name: "OLPC 565",
    extensions: &["565"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
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
