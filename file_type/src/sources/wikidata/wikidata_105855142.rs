use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855142: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_142,
        source_type: SourceType::Wikidata,
        name: "HxC Floppy Emulator Floppy Profile (v0.1)",
        extensions: &["fpf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x50, 0x46, 0x5F, 0x56, 0x30, 0x2E, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
