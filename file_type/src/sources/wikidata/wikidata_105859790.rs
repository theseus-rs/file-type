use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859790: FileFormat = FileFormat {
    id: 105_859_790,
    source_type: SourceType::Wikidata,
    name: "Vectric Cut3D model",
    extensions: &["v3d"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
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
