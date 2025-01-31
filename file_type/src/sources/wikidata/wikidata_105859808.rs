use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859808: FileFormat = FileFormat {
    id: 105_859_808,
    puid: "wikidata/105859808",
    name: "Var Bitmap Font (v2)",
    extensions: &["vbf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x76, 0x62, 0x66, 0x2D, 0x32, 0x2E, 0x30, 0x2A, 0x2A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
