use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860365: FileFormat = FileFormat {
    id: 105_860_365,
    puid: "wikidata/105860365",
    name: "File Crypt encrypted",
    extensions: &["rzx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xB4, 0xB2, 0x6F, 0x93, 0xE2, 0xA6, 0x90, 0x07, 0x6F, 0xC9, 0xCC, 0x2C, 0x1A,
                    0xEB, 0xB9, 0x24,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
