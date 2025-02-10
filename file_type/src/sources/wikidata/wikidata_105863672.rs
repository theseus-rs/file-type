use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863672: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_672,
        source_type: SourceType::Wikidata,
        name: "Palm Memo Pad Archive",
        extensions: &["mpa"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x01, 0x50, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
