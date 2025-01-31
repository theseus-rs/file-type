use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858877: FileFormat = FileFormat {
    id: 105_858_877,
    puid: "wikidata/105858877",
    name: "BIMx 3D scene",
    extensions: &["bimx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x49, 0x4D, 0x78, 0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00,
                    0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
