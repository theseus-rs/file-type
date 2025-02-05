use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856091: FileFormat = FileFormat {
    id: 105_856_091,
    source_type: SourceType::Wikidata,
    name: "Dynamic Studio Professional module",
    extensions: &["dsm"],
    media_types: &["audio/x-mod"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x53, 0x6D, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
