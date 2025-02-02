use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857508: FileFormat = FileFormat {
    id: 105_857_508,
    source_type: SourceType::Wikidata,
    name: "Destan game data archive",
    extensions: &["3dn"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x65, 0x73, 0x74, 0x61, 0x6E, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x66,
                    0x6F, 0x72, 0x6D, 0x61, 0x74, 0x2C, 0x20, 0x4D, 0x69, 0x63, 0x68, 0x61, 0x6C,
                    0x20, 0x54, 0x61, 0x74, 0x6B, 0x61, 0x20, 0x32, 0x30, 0x30, 0x35,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
