use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858113: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_113,
        source_type: SourceType::Wikidata,
        name: "Infinity Engine configuration",
        extensions: &["ini"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x6C, 0x6F, 0x63, 0x61, 0x6C, 0x73, 0x5D, 0x0D, 0x0A, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
