use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852096: FileFormat = FileFormat {
    id: 105_852_096,
    source_type: SourceType::Wikidata,
    name: "Weather data summary report",
    extensions: &["stat"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x20, 0x53, 0x74, 0x61, 0x74, 0x69, 0x73, 0x74, 0x69, 0x63, 0x73, 0x20, 0x66,
                    0x6F, 0x72, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
