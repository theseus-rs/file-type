use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854203: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_203,
        source_type: SourceType::Wikidata,
        name: "FunCom ISS audio",
        extensions: &["iss", "xarc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x4D, 0x41, 0x5F, 0x41, 0x44, 0x50, 0x43, 0x4D, 0x5F, 0x53, 0x6F,
                        0x75, 0x6E, 0x64,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
