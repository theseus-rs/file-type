use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849929: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_929,
        source_type: SourceType::Wikidata,
        name: "HxC Floppy Emulator configuration (v1.0)",
        extensions: &["cfg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x58, 0x43, 0x46, 0x45, 0x43, 0x46, 0x47, 0x56, 0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
