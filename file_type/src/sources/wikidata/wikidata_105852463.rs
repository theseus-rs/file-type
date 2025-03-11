use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852463: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_463,
        source_type: SourceType::Wikidata,
        name: "PowerplayerMusic Cruncher (PMC) (v2.x)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x46, 0x43, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
