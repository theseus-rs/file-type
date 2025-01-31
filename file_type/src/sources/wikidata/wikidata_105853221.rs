use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853221: FileFormat = FileFormat {
    id: 105_853_221,
    puid: "wikidata/105853221",
    name: "FreeDOS KEYBoard layout collection",
    extensions: &["sys"],
    media_types: &["application/x-fdos-keyb"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4B, 0x43, 0x46, 0x00, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
