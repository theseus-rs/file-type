use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867579: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_579,
        source_type: SourceType::Wikidata,
        name: "NUnit project",
        extensions: &["nunit"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEF, 0xBB, 0xBF, 0x3C, 0x4E, 0x55, 0x6E, 0x69, 0x74, 0x50, 0x72, 0x6F,
                        0x6A, 0x65, 0x63, 0x74, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
