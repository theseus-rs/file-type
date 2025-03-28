use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863896: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_896,
        source_type: SourceType::Wikidata,
        name: "thinEdge model",
        extensions: &["m15"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x27, 0x74, 0x68, 0x69, 0x6E, 0x45, 0x64, 0x67, 0x65, 0x20, 0x64, 0x61,
                        0x74, 0x61, 0x66, 0x69, 0x6C, 0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
