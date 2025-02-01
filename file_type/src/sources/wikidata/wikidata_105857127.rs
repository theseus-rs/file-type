use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857127: FileFormat = FileFormat {
    id: 105_857_127,
    puid: "wikidata/105857127",
    name: "Opticks Gel",
    extensions: &["gel"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0A, 0x47, 0x45, 0x4C, 0x76, 0x31, 0x2E, 0x30, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
