use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856024: FileFormat = FileFormat {
    id: 105_856_024,
    puid: "wikidata/105856024",
    name: "3DFX Glide driver",
    extensions: &["dxe"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x58, 0x45, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
