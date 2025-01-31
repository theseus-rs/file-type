use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67441966: FileFormat = FileFormat {
    id: 67_441_966,
    puid: "wikidata/67441966",
    name: "Robocode Battle",
    extensions: &["battle"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x42, 0x61, 0x74, 0x74, 0x6C, 0x65, 0x20, 0x50, 0x72, 0x6F, 0x70, 0x65,
                    0x72, 0x74, 0x69, 0x65, 0x73,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
