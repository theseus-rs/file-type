use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857278: FileFormat = FileFormat {
    id: 105_857_278,
    puid: "wikidata/105857278",
    name: "HNSKY Deep Sky Database",
    extensions: &["hnd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x4E, 0x53, 0x4B, 0x59, 0x20, 0x44, 0x45, 0x45, 0x50, 0x53, 0x4B, 0x59,
                    0x20, 0x44, 0x41, 0x54, 0x41, 0x42, 0x41, 0x53, 0x45, 0x20, 0x4C, 0x45, 0x56,
                    0x45, 0x4C, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
