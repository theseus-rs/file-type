use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861757: FileFormat = FileFormat {
    id: 105_861_757,
    source_type: SourceType::Wikidata,
    name: "Microsoft Money data",
    extensions: &["mny"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x01, 0x00, 0x00, 0x4D, 0x53, 0x49, 0x53, 0x41, 0x4D, 0x20, 0x44, 0x61,
                    0x74, 0x61, 0x62, 0x61, 0x73, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
