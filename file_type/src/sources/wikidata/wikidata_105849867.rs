use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849867: FileFormat = FileFormat {
    id: 105_849_867,
    source_type: SourceType::Wikidata,
    name: "Vernier Logger Pro data",
    extensions: &["cmbl"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x44, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
