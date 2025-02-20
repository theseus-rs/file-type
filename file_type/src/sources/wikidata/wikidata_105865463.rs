use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865463: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_463,
        source_type: SourceType::Wikidata,
        name: "CWPAL color palette",
        extensions: &["pal"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x57, 0x50, 0x41, 0x4C, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
