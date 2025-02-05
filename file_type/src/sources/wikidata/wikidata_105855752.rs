use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855752: FileFormat = FileFormat {
    id: 105_855_752,
    source_type: SourceType::Wikidata,
    name: "GBG DraftMaker V6.0 drawing",
    extensions: &["d"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x7D, 0x20, 0x39, 0x47, 0x42, 0x47, 0x2D, 0x56, 0x7B, 0x36, 0x2E, 0x7B, 0x30,
                    0x7B, 0x30, 0x3B, 0x47, 0x42, 0x4C, 0x49, 0x42, 0x2D, 0x56, 0x7B, 0x36, 0x2E,
                    0x7B, 0x30, 0x7B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
