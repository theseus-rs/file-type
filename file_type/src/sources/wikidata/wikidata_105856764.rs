use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856764: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_764,
        source_type: SourceType::Wikidata,
        name: "HxC Floppy Emulator firmware Update (Gotek)",
        extensions: &["upd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x58, 0x43, 0x46, 0x55, 0x50, 0x44, 0x41, 0x54, 0x45, 0x00, 0x04,
                        0x00, 0x43, 0x58, 0x46, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
