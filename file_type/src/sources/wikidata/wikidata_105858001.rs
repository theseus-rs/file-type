use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858001: FileFormat = FileFormat {
    id: 105_858_001,
    source_type: SourceType::Wikidata,
    name: "DB/TextWorks Database Indexed List",
    extensions: &["ixl"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x58, 0x4C, 0x20, 0x30, 0x30, 0x32, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
