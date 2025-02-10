use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858569: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_569,
        source_type: SourceType::Wikidata,
        name: "Binary Color Format",
        extensions: &["bcf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x43, 0x46, 0x20, 0x31, 0x2E, 0x30, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
