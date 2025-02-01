use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849786: FileFormat = FileFormat {
    id: 105_849_786,
    puid: "wikidata/105849786",
    name: "Cal3D Animation File",
    extensions: &["caf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x41, 0x46, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
