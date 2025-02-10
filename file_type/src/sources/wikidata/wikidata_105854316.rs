use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854316: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_316,
        source_type: SourceType::Wikidata,
        name: "Velvet Studio Advanced Module System module",
        extensions: &["ams"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x4D, 0x53, 0x68, 0x64, 0x72, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
