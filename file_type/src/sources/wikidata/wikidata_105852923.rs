use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852923: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_923,
        source_type: SourceType::Wikidata,
        name: "Hondata S-Manager calibration",
        extensions: &["skl"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3B, 0x0D, 0x0A, 0x3B, 0x20, 0x48, 0x6F, 0x6E, 0x64, 0x61, 0x74, 0x61,
                        0x20, 0x53, 0x2D, 0x4D, 0x61, 0x6E, 0x61, 0x67, 0x65, 0x72, 0x20, 0x43,
                        0x61, 0x6C, 0x69, 0x62, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
