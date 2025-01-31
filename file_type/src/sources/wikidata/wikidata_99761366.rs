use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_99761366: FileFormat = FileFormat {
    id: 99_761_366,
    puid: "wikidata/99761366",
    name: "Canon Original RAW, version 3",
    extensions: &["cr3"],
    media_types: &["image/x-canon-cr3"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, 0x63, 0x72, 0x78, 0x20, 0x00,
                    0x00, 0x00, 0x01, 0x63, 0x72, 0x78, 0x20, 0x69, 0x73, 0x6F, 0x6D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
