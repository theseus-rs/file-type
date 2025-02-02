use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865239: FileFormat = FileFormat {
    id: 105_865_239,
    source_type: SourceType::Wikidata,
    name: "PowerBASIC/DOS configuration file",
    extensions: &["pb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x6F, 0x77, 0x65, 0x72, 0x42, 0x61, 0x73, 0x69, 0x63, 0x20, 0x43, 0x6F,
                    0x6E, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x46,
                    0x69, 0x6C, 0x65, 0x20, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
