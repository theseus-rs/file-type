use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600756: FileFormat = FileFormat {
    id: 28_600_756,
    puid: "wikidata/28600756",
    name: "ER Mapper Vector",
    extensions: &["erv"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x61, 0x74, 0x61, 0x73, 0x65, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72,
                    0x20, 0x42, 0x65, 0x67, 0x69, 0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
