use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851883: FileFormat = FileFormat {
    id: 105_851_883,
    source_type: SourceType::Wikidata,
    name: "SMS Super File",
    extensions: &["sup"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x55, 0x50, 0x45, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
