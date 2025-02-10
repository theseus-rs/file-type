use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_307271: FileType = FileType {
    file_format: &FileFormat {
        id: 307_271,
        source_type: SourceType::Wikidata,
        name: ".DS_Store",
        extensions: &["DS_Store"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x01, 0x42, 0x75, 0x64, 0x31, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
