use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855372: FileFormat = FileFormat {
    id: 105_855_372,
    source_type: SourceType::Wikidata,
    name: "Blueberry FlashBack screen Record",
    extensions: &["fbr"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x1F, 0x8B, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0B, 0x0B, 0x60, 0x60,
                    0x60, 0x60, 0x61, 0x20, 0x1F, 0x30, 0x33, 0xB0, 0xA2, 0xF0, 0xFF, 0xFF, 0xFF,
                    0x5F, 0xFF, 0x9F, 0x38, 0x40, 0x55, 0x75, 0x00, 0x37, 0xB5, 0xA7, 0x91, 0xC8,
                    0x00, 0x00, 0x00, 0x87, 0x63, 0x95, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
