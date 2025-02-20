use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862363: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_363,
        source_type: SourceType::Wikidata,
        name: "MEGA data format",
        extensions: &["meg"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x6D, 0x65, 0x67, 0x61])],
                },
            }],
        }],
        related_formats: &[],
    },
};
