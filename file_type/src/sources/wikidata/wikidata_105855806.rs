use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855806: FileFormat = FileFormat {
    id: 105_855_806,
    source_type: SourceType::Wikidata,
    name: "DB/TextWorks Database Directory",
    extensions: &["dbo"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x42, 0x4F, 0x20, 0x30, 0x31, 0x32, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
