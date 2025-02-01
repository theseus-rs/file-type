use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28771225: FileFormat = FileFormat {
    id: 28_771_225,
    puid: "wikidata/28771225",
    name: "MAX (3ds Max)",
    extensions: &["max"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
