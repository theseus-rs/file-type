use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860079: FileFormat = FileFormat {
    id: 105_860_079,
    puid: "wikidata/105860079",
    name: "Var Bitmap Font (v1)",
    extensions: &["vbf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x76, 0x62, 0x66, 0x2D, 0x31, 0x2E, 0x30, 0x2A, 0x2A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
