use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860272: FileFormat = FileFormat {
    id: 105_860_272,
    puid: "wikidata/105860272",
    name: "Resume Builder 4.x",
    extensions: &["rb4"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x42, 0x34, 0x00, 0x28, 0x63, 0x29, 0x20, 0x53, 0x61, 0x72, 0x6D, 0x20,
                    0x53, 0x6F, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
