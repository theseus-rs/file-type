use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128123326: FileFormat = FileFormat {
    id: 128_123_326,
    puid: "wikidata/128123326",
    name: "A2R disk image (v3)",
    extensions: &["a2r"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x32, 0x52, 0x33, 0xFF, 0x0A, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
