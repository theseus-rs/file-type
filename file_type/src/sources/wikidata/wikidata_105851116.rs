use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851116: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_116,
        source_type: SourceType::Wikidata,
        name: "Type Library (Type2)",
        extensions: &["tlb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x53, 0x46, 0x54, 0x02, 0x00, 0x01, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
