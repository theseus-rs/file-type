use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858091: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_091,
        source_type: SourceType::Wikidata,
        name: "InDesign Shortcuts set",
        extensions: &["indk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x50, 0x50, 0x50, 0x43, 0x53, 0x42, 0x4B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
