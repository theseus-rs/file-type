use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859115: FileFormat = FileFormat {
    id: 105_859_115,
    puid: "wikidata/105859115",
    name: "Taquart Interlace Picture bitmap",
    extensions: &["tip"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x49, 0x50, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
