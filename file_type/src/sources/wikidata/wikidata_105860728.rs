use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860728: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_728,
        source_type: SourceType::Wikidata,
        name: "ProvideX Report",
        extensions: &["rpt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x50, 0x76, 0x78, 0x72, 0x70, 0x74, 0x5D, 0x0D, 0x0A, 0x55, 0x73,
                        0x65, 0x72, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
