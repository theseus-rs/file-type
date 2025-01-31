use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857699: FileFormat = FileFormat {
    id: 105_857_699,
    puid: "wikidata/105857699",
    name: "iOS crash report",
    extensions: &["txt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x6E, 0x63, 0x69, 0x64, 0x65, 0x6E, 0x74, 0x20, 0x49, 0x64, 0x65, 0x6E,
                    0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x3A, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
