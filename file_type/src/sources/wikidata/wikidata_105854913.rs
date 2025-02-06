use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854913: FileFormat = FileFormat {
    id: 105_854_913,
    source_type: SourceType::Wikidata,
    name: "STOS Sample",
    extensions: &["sam"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x4F, 0x4E])],
            },
        }],
    }],
    related_formats: &[],
};
