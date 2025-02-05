use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857296: FileFormat = FileFormat {
    id: 105_857_296,
    source_type: SourceType::Wikidata,
    name: "Harry Potter And The Sorcerers Stone game data archive",
    extensions: &["hog"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x41, 0x52, 0x54, 0x33, 0x2E, 0x30, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
