use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854127: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_127,
        source_type: SourceType::Wikidata,
        name: "Alpha Five Web Components",
        extensions: &["a5wcmp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x35, 0x42, 0x4C, 0x4F, 0x42, 0x53, 0x54, 0x00, 0x02, 0x00, 0x00,
                        0x1C, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
