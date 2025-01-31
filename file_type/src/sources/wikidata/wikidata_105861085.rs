use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861085: FileFormat = FileFormat {
    id: 105_861_085,
    puid: "wikidata/105861085",
    name: "PCB Layout",
    extensions: &["lyt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x43, 0x42, 0x20, 0x4C, 0x41, 0x59, 0x4F, 0x55, 0x54, 0x20, 0x46, 0x49,
                    0x4C, 0x45, 0x1A, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
