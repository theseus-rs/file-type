use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856969: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_969,
        source_type: SourceType::Wikidata,
        name: "Genbox Family History report options",
        extensions: &["gro"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x42, 0x4F, 0x58, 0x52, 0x4F, 0x50, 0x54,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
