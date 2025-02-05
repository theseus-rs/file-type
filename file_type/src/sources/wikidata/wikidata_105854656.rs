use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854656: FileFormat = FileFormat {
    id: 105_854_656,
    source_type: SourceType::Wikidata,
    name: "PEA compressed archive (v1.x)",
    extensions: &["pea"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xEA, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
