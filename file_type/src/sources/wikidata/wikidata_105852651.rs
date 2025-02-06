use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852651: FileFormat = FileFormat {
    id: 105_852_651,
    source_type: SourceType::Wikidata,
    name: "SGP Baltie Program",
    extensions: &["bpr"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x50, 0x52, 0x03])],
            },
        }],
    }],
    related_formats: &[],
};
