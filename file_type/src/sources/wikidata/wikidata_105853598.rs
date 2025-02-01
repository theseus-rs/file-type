use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853598: FileFormat = FileFormat {
    id: 105_853_598,
    puid: "wikidata/105853598",
    name: "Minecraft game data",
    extensions: &["zipe"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x00, 0xFC, 0xB9, 0xCF, 0x9B, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x24,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
