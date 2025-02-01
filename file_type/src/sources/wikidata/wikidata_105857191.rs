use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857191: FileFormat = FileFormat {
    id: 105_857_191,
    puid: "wikidata/105857191",
    name: "HOOPS 3D Stream Format",
    extensions: &["hsf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3B, 0x3B, 0x20, 0x48, 0x53, 0x46, 0x20, 0x56,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
