use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861490: FileFormat = FileFormat {
    id: 105_861_490,
    puid: "wikidata/105861490",
    name: "The Movies Editor Text String Database",
    extensions: &["lhts"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
