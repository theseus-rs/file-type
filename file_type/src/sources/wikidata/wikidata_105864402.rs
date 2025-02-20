use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864402: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_402,
        source_type: SourceType::Wikidata,
        name: "Pixelformer Colors",
        extensions: &["pfcolors"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x49, 0x58, 0x45, 0x4C, 0x46, 0x4F, 0x52, 0x4D, 0x45, 0x52, 0x2E,
                        0x43, 0x4F, 0x4C, 0x4F, 0x52, 0x53, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
