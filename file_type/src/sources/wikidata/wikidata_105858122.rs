use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858122: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_122,
        source_type: SourceType::Wikidata,
        name: "QDOS QL floppy disk image (DS/HD)",
        extensions: &["img"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x51, 0x4C, 0x35, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
