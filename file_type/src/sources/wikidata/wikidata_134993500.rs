use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_134993500: FileType = FileType {
    file_format: &FileFormat {
        id: 134_993_500,
        source_type: SourceType::Wikidata,
        name: "Interchangeable Preservation Format",
        extensions: &["ipf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x41, 0x50, 0x53, 0x00, 0x00, 0x00, 0x0C, 0x1C, 0xD5, 0x73, 0xBA,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
