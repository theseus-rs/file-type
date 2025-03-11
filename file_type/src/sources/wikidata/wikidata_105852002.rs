use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852002: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_002,
        source_type: SourceType::Wikidata,
        name: "SpecBAS Ascii text",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5A, 0x58, 0x41, 0x53, 0x43, 0x49, 0x49])],
                },
            }],
        }],
        related_formats: &[],
    },
};
