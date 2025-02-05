use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857653: FileFormat = FileFormat {
    id: 105_857_653,
    source_type: SourceType::Wikidata,
    name: "the Word Bible Text Module Index",
    extensions: &["idx"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x54, 0x57, 0x49, 0x44, 0x58, 0x42, 0x4D, 0x4F, 0x44,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
