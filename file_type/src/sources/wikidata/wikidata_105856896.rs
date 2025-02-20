use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856896: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_896,
        source_type: SourceType::Wikidata,
        name: "GFI Backup Task",
        extensions: &["gbt"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x58, 0x4D, 0x4C, 0x3E, 0x0D, 0x0A, 0x20, 0x20, 0x3C, 0x47, 0x45,
                        0x4E, 0x45, 0x52, 0x41, 0x4C, 0x3E, 0x0D, 0x0A, 0x20, 0x20, 0x20, 0x20,
                        0x3C, 0x54, 0x79, 0x70, 0x65, 0x20, 0x62, 0x43, 0x6F, 0x75, 0x6E, 0x74,
                        0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
