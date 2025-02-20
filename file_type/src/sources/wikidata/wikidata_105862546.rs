use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862546: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_546,
        source_type: SourceType::Wikidata,
        name: "MCMD module",
        extensions: &["mcmd"],
        media_types: &["audio/x-mod"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x43, 0x4D, 0x44, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
