use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857109: FileFormat = FileFormat {
    id: 105_857_109,
    puid: "wikidata/105857109",
    name: "ZBrush GoZ export template",
    extensions: &["goz"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x5A, 0x42, 0x72, 0x75, 0x73, 0x68, 0x2C, 0x47, 0x4F, 0x5A, 0x5F, 0x54,
                    0x41, 0x47, 0x5F, 0x48, 0x45, 0x41, 0x44, 0x45, 0x52, 0x5D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
