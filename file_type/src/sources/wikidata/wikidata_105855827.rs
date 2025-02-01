use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855827: FileFormat = FileFormat {
    id: 105_855_827,
    puid: "wikidata/105855827",
    name: "Dart Desktop",
    extensions: &["dsk"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x61, 0x72, 0x74, 0x20, 0x64, 0x65, 0x73, 0x6B, 0x74, 0x6F, 0x70, 0x20,
                    0x66, 0x69, 0x6C, 0x65, 0x2E, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
