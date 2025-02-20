use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860792: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_792,
        source_type: SourceType::Wikidata,
        name: "Family Origins Report (v5.x)",
        extensions: &["rpt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x4F, 0x57, 0x49, 0x4E, 0x5F, 0x52, 0x45, 0x50, 0x4F, 0x52, 0x54,
                        0x0C, 0x34, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
