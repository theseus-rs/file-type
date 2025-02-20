use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862176: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_176,
        source_type: SourceType::Wikidata,
        name: "MusicIndiaOnline Trident Player Music file",
        extensions: &["mia"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x49, 0x4F, 0x41, 0x4C, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
