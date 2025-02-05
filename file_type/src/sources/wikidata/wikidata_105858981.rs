use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858981: FileFormat = FileFormat {
    id: 105_858_981,
    source_type: SourceType::Wikidata,
    name: "STJ Stereoscopic bitmap",
    extensions: &["stj"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x00,
                    0x01, 0x00, 0x96, 0x00, 0x96, 0x00, 0x00, 0xFF, 0xE5, 0x00, 0x08, 0x33, 0x44,
                    0x70, 0x69, 0x63, 0x00, 0xFF, 0xFE, 0x00, 0x0B, 0x63, 0x6F, 0x70, 0x79, 0x3D,
                    0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
