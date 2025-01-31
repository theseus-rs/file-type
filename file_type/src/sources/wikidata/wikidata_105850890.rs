use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850890: FileFormat = FileFormat {
    id: 105_850_890,
    puid: "wikidata/105850890",
    name: "KISSSlicer Project",
    extensions: &["ksp"],
    media_types: &["text/ini"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3B, 0x0D, 0x0A, 0x3B, 0x20, 0x2A, 0x2A, 0x2A, 0x20, 0x50, 0x72, 0x69, 0x6E,
                    0x74, 0x65, 0x72, 0x20, 0x53, 0x65, 0x74, 0x74, 0x69, 0x6E, 0x67, 0x73, 0x20,
                    0x2A, 0x2A, 0x2A, 0x0D, 0x0A, 0x3B, 0x0D, 0x0A, 0x3B, 0x20, 0x70, 0x72, 0x69,
                    0x6E, 0x74, 0x65, 0x72, 0x5F, 0x6E, 0x61, 0x6D, 0x65, 0x20, 0x3D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
