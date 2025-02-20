use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858393: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_393,
        source_type: SourceType::Wikidata,
        name: "eDonkey network download link",
        extensions: &["ed2k"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x65, 0x64, 0x32, 0x6B, 0x3A, 0x2F, 0x2F, 0x7C, 0x66, 0x69, 0x6C, 0x65,
                        0x7C,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
