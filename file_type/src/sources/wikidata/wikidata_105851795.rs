use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851795: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_795,
        source_type: SourceType::Wikidata,
        name: "WinAPE Snapshot (v3)",
        extensions: &["sna"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x56, 0x20, 0x2D, 0x20, 0x53, 0x4E, 0x41, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x03,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
