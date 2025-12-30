use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125019957: FileType = FileType {
    file_format: &FileFormat {
        id: 125_019_957,
        source_type: SourceType::Wikidata,
        name: "GrandView Outline file",
        extensions: &["gv"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x1A, 0x4A, 0x4C, 0x46, 0x5F, 0x49, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
