use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858656: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_656,
        source_type: SourceType::Wikidata,
        name: "TomTom Go info",
        extensions: &["bif"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x54, 0x6F, 0x6D, 0x54, 0x6F, 0x6D, 0x47, 0x6F, 0x5D, 0x0D, 0x0A,
                        0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x4E, 0x61, 0x6D, 0x65, 0x3D, 0x54,
                        0x6F, 0x6D, 0x54, 0x6F, 0x6D, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
