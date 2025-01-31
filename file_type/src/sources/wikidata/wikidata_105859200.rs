use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859200: FileFormat = FileFormat {
    id: 105_859_200,
    puid: "wikidata/105859200",
    name: "GETIC 3D BSP",
    extensions: &["bsp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x47, 0x42, 0x54, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
