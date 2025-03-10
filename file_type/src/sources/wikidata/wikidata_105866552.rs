use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866552: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_552,
        source_type: SourceType::Wikidata,
        name: "ProCONTROL script",
        extensions: &[],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x72, 0x6F, 0x43, 0x4F, 0x4E, 0x54, 0x52, 0x4F, 0x4C, 0x20, 0x73,
                        0x63, 0x72, 0x69, 0x70, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
