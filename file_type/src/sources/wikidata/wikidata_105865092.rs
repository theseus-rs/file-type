use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865092: FileFormat = FileFormat {
    id: 105_865_092,
    source_type: SourceType::Wikidata,
    name: "PowerBuilder Workspace",
    extensions: &["pbw"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x61, 0x76, 0x65, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x20, 0x76,
                    0x33, 0x2E, 0x30, 0x28, 0x31, 0x39, 0x39, 0x39, 0x30, 0x31, 0x31, 0x32, 0x29,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
