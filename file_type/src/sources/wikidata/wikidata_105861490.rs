use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861490: FileFormat = FileFormat {
    id: 105_861_490,
    source_type: SourceType::Wikidata,
    name: "The Movies Editor Text String Database",
    extensions: &["lhts"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x48, 0x54, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
