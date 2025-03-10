use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859156: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_156,
        source_type: SourceType::Wikidata,
        name: "Borland Form (generic)",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x50, 0x46, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
