use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864182: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_182,
        source_type: SourceType::Wikidata,
        name: "Pixel image",
        extensions: &["px"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
