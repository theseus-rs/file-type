use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860504: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_504,
        source_type: SourceType::Wikidata,
        name: "ATG Core Cement Format game data archive",
        extensions: &["rcf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x54, 0x47, 0x20, 0x43, 0x4F, 0x52, 0x45, 0x20, 0x43, 0x45, 0x4D,
                        0x45, 0x4E, 0x54, 0x20, 0x4C, 0x49, 0x42, 0x52, 0x41, 0x52, 0x59, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
