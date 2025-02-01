use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856944: FileFormat = FileFormat {
    id: 105_856_944,
    puid: "wikidata/105856944",
    name: "Granite Devices Firmware (v300)",
    extensions: &["gdf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x44, 0x46, 0x57, 0x2C, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
