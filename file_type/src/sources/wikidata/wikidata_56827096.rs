use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_56827096: FileType = FileType {
    file_format: &FileFormat {
        id: 56_827_096,
        source_type: SourceType::Wikidata,
        name: "Web Assembly Binary Format",
        extensions: &["wasm"],
        media_types: &["application/octet-stream", "application/wasm"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x61, 0x73, 0x6D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
