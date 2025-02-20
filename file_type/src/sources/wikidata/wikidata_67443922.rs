use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67443922: FileType = FileType {
    file_format: &FileFormat {
        id: 67_443_922,
        source_type: SourceType::Wikidata,
        name: "Black and White 2 Environment data",
        extensions: &["bwe"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x28,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
