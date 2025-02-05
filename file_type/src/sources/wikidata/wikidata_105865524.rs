use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865524: FileFormat = FileFormat {
    id: 105_865_524,
    source_type: SourceType::Wikidata,
    name: "Across crossword puzzle",
    extensions: &["puz"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x43, 0x52, 0x4F, 0x53, 0x53, 0x26, 0x44, 0x4F, 0x57, 0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
