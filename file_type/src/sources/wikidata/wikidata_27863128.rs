use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27863128: FileType = FileType {
    file_format: &FileFormat {
        id: 27_863_128,
        source_type: SourceType::Wikidata,
        name: "AutoCAD R14 Drawing (subtype 13)",
        extensions: &["dwg"],
        media_types: &["application/x-autocad", "image/vnd.dwg"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0xAC, 0x10, 0x13])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x41, 0x43, 0x31, 0x30, 0x31, 0x33])],
                    },
                }],
            },
        ],
        related_formats: &[],
    },
};
