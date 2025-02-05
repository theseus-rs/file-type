use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854306: FileFormat = FileFormat {
    id: 105_854_306,
    source_type: SourceType::Wikidata,
    name: "PR archiving tool archive",
    extensions: &["ar"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x52, 0x0D])],
            },
        }],
    }],
    related_formats: &[],
};
