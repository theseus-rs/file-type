use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105864230: FileFormat = FileFormat {
    id: 105_864_230,
    puid: "wikidata/105864230",
    name: "PowerBASIC Help",
    extensions: &["pbh"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x72, 0x7A, 0x6A, 0x66, 0x68, 0x65, 0x6C, 0x70,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
