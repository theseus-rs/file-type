use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105862542: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_542,
        source_type: SourceType::Wikidata,
        name: "OS/2 help Message (generic)",
        extensions: &["msg"],
        media_types: &["application/x-os2-msg"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0x4D, 0x4B, 0x4D, 0x53, 0x47, 0x46, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
