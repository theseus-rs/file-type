use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851931: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_931,
        source_type: SourceType::Wikidata,
        name: "BrMSX savestate",
        extensions: &["sta"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x53, 0x58, 0x53, 0x54, 0x41, 0x54, 0x45, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
