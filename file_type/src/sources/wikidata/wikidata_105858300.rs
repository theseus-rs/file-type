use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858300: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_300,
        source_type: SourceType::Wikidata,
        name: "Autex Experimental Pole Figure data",
        extensions: &["exp"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x55, 0x54, 0x45, 0x58, 0x2D, 0x45, 0x58, 0x50, 0x45, 0x52, 0x49,
                        0x4D, 0x45, 0x4E, 0x54, 0x41, 0x4C, 0x20, 0x50, 0x4F, 0x4C, 0x45, 0x20,
                        0x46, 0x49, 0x47, 0x55, 0x52, 0x45, 0x20, 0x44, 0x41, 0x54, 0x41,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
