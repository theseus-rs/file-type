use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_73504409: FileType = FileType {
    file_format: &FileFormat {
        id: 73_504_409,
        source_type: SourceType::Wikidata,
        name: "ABC FlowCharter file format",
        extensions: &["af2", "af3"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x18, 0x00, 0x4A, 0x46, 0x4F, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
