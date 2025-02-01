use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852559: FileFormat = FileFormat {
    id: 105_852_559,
    puid: "wikidata/105852559",
    name: "STT disk image",
    extensions: &["stt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x54, 0x45, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
