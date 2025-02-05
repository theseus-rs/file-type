use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852401: FileFormat = FileFormat {
    id: 105_852_401,
    source_type: SourceType::Wikidata,
    name: "Solace Virtual Northstar disk image",
    extensions: &["svn"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x56, 0x44, 0x3A, 0x53, 0x6F, 0x6C, 0x61, 0x63, 0x65, 0x20, 0x56, 0x69,
                    0x72, 0x74, 0x75, 0x61, 0x6C, 0x20, 0x44, 0x69, 0x73, 0x6B, 0x2C, 0x20, 0x4E,
                    0x6F, 0x72, 0x74, 0x68, 0x73, 0x74, 0x61, 0x72,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
