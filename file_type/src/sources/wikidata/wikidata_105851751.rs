use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851751: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_751,
        source_type: SourceType::Wikidata,
        name: "Project Master Statistic data",
        extensions: &["sta"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x52, 0x4F, 0x4A, 0x45, 0x43, 0x54, 0x20, 0x44, 0x41, 0x54, 0x41,
                        0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
