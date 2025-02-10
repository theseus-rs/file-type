use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852832: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_832,
        source_type: SourceType::Wikidata,
        name: "SharkPort file",
        extensions: &["sps"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0D, 0x00, 0x00, 0x00, 0x53, 0x68, 0x61, 0x72, 0x6B, 0x50, 0x6F, 0x72,
                        0x74, 0x53, 0x61, 0x76, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
