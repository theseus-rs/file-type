use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851576: FileFormat = FileFormat {
    id: 105_851_576,
    source_type: SourceType::Wikidata,
    name: "Peria Chronicles game data archive",
    extensions: &["tarc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x68, 0x69, 0x6E, 0x67, 0x20, 0x41, 0x72, 0x63, 0x68, 0x69, 0x76, 0x65,
                    0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
