use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856469: FileFormat = FileFormat {
    id: 105_856_469,
    source_type: SourceType::Wikidata,
    name: "WinWorks text Document",
    extensions: &["wpd"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x49, 0x4E, 0x57, 0x4F, 0x52, 0x4B, 0x53, 0x20, 0x44, 0x4F, 0x43, 0x55,
                    0x4D, 0x45, 0x4E, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
