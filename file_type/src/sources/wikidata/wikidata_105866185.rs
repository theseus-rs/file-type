use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866185: FileFormat = FileFormat {
    id: 105_866_185,
    puid: "wikidata/105866185",
    name: "Bitmapped Signum!2 printer font (24 Pins)",
    extensions: &["p24"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x70, 0x73, 0x32, 0x34, 0x30, 0x30, 0x30, 0x31, 0x00, 0x00, 0x00, 0x80,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
