use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861571: FileFormat = FileFormat {
    id: 105_861_571,
    puid: "wikidata/105861571",
    name: "BML3MK5 recorded keys",
    extensions: &["l3k"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4B, 0x45, 0x59, 0x52, 0x45, 0x43, 0x4F, 0x52, 0x44, 0x5F, 0x42, 0x4D, 0x4C,
                    0x33, 0x4D, 0x4B, 0x35,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
