use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860606: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_606,
        source_type: SourceType::Wikidata,
        name: "ReportMachine Prepared report",
        extensions: &["rmp"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x00, 0x52, 0x65, 0x70, 0x6F, 0x72, 0x74, 0x4D, 0x61, 0x63, 0x68,
                        0x69, 0x6E, 0x65, 0x20, 0x50, 0x72, 0x65, 0x70, 0x61, 0x72, 0x65, 0x64,
                        0x20, 0x52, 0x65, 0x70, 0x6F, 0x72, 0x74, 0x20, 0x46, 0x69, 0x6C, 0x65,
                        0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
