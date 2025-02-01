use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859021: FileFormat = FileFormat {
    id: 105_859_021,
    puid: "wikidata/105859021",
    name: "Message string storage",
    extensions: &["bmg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x45, 0x53, 0x47, 0x62, 0x6D, 0x67, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
