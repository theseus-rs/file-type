use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866228: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_228,
        source_type: SourceType::Wikidata,
        name: "IntroCAD Printer Definition",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x72, 0x74, 0x44, 0x65, 0x66, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
