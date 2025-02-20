use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855272: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_272,
        source_type: SourceType::Wikidata,
        name: "FIGfont control file",
        extensions: &["flc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x66, 0x6C, 0x63, 0x32, 0x61])],
                },
            }],
        }],
        related_formats: &[],
    },
};
