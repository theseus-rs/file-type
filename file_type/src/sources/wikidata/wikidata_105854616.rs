use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854616: FileFormat = FileFormat {
    id: 105_854_616,
    puid: "wikidata/105854616",
    name: "DS Squeeze archive",
    extensions: &["ark"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x49, 0x44, 0x46, 0x55, 0x47, 0x48, 0x54, 0x41, 0xD5, 0x02,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
