use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861220: FileFormat = FileFormat {
    id: 105_861_220,
    puid: "wikidata/105861220",
    name: "Bitmapped Signum!2 printer font (Laser/Inkjet)",
    extensions: &["l30"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x6C, 0x73, 0x33, 0x30, 0x30, 0x30, 0x30, 0x31, 0x00, 0x00, 0x00, 0x80,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
