use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852137: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_137,
        source_type: SourceType::Wikidata,
        name: "ABC SnapGraphics Palette",
        extensions: &["sgp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x1D, 0x00, 0x47, 0x46, 0x2A, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
