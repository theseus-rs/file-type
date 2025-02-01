use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856140: FileFormat = FileFormat {
    id: 105_856_140,
    puid: "wikidata/105856140",
    name: "ISIS Schematic file",
    extensions: &["dsn"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x53, 0x49, 0x53, 0x20, 0x53, 0x43, 0x48, 0x45, 0x4D, 0x41, 0x54, 0x49,
                    0x43, 0x20, 0x46, 0x49, 0x4C, 0x45, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
