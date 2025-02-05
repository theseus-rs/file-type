use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856527: FileFormat = FileFormat {
    id: 105_856_527,
    source_type: SourceType::Wikidata,
    name: "WordStar Macro",
    extensions: &["wsm"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x75, 0x62, 0x20, 0x4D, 0x61, 0x69, 0x6E, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
