use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105592581: FileType = FileType {
    file_format: &FileFormat {
        id: 105_592_581,
        source_type: SourceType::Wikidata,
        name: "TuxGuitar",
        extensions: &["tg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x54, 0x00, 0x75, 0x00, 0x78, 0x00, 0x47, 0x00, 0x75, 0x00, 0x69,
                        0x00, 0x74, 0x00, 0x61, 0x00, 0x72, 0x00, 0x20, 0x00, 0x46, 0x00, 0x69,
                        0x00, 0x6C, 0x00, 0x65, 0x00, 0x20, 0x00, 0x46, 0x00, 0x6F, 0x00, 0x72,
                        0x00, 0x6D, 0x00, 0x61, 0x00, 0x74, 0x00, 0x20, 0x00, 0x2D, 0x00, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
