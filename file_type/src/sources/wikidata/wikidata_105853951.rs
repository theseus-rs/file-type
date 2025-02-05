use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853951: FileFormat = FileFormat {
    id: 105_853_951,
    source_type: SourceType::Wikidata,
    name: "sr2 compressed data",
    extensions: &["sr2"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x52, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
