use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864504: FileFormat = FileFormat {
    id: 105_864_504,
    puid: "wikidata/105864504",
    name: "PROGRESS Procedure Library (v11)",
    extensions: &["pl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xD7, 0x0B, 0x07])],
            },
        }],
    }],
    related_formats: &[],
};
