use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853268: FileFormat = FileFormat {
    id: 105_853_268,
    source_type: SourceType::Wikidata,
    name: "SPSS template",
    extensions: &["cht", "sct"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x04, 0x00, 0x00, 0x53, 0x50, 0x53, 0x53, 0x20, 0x74, 0x65, 0x6D, 0x70,
                    0x6C, 0x61, 0x74, 0x65, 0x20, 0x66, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
