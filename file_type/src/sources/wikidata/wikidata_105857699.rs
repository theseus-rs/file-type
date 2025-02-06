use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857699: FileFormat = FileFormat {
    id: 105_857_699,
    source_type: SourceType::Wikidata,
    name: "iOS crash report",
    extensions: &["txt"],
    media_types: &[],
    signatures: &[Signature {
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
