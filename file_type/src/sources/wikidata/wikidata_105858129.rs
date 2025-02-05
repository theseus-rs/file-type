use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858129: FileFormat = FileFormat {
    id: 105_858_129,
    source_type: SourceType::Wikidata,
    name: "AOL thumbnails index",
    extensions: &["ind"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x4F, 0x4C, 0x49, 0x44, 0x58, 0x4B, 0x32,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
