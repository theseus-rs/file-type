use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207237: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_237,
        source_type: SourceType::Wikidata,
        name: "Multiple Resolution Bitmap",
        extensions: &["mrb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6C, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
