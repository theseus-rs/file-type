use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859538: FileFormat = FileFormat {
    id: 105_859_538,
    puid: "wikidata/105859538",
    name: "Webcam Video Effects pack",
    extensions: &["vfz"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x50, 0x47, 0x01, 0x00, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
