use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858588: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_588,
        source_type: SourceType::Wikidata,
        name: "PM XV bitmap",
        extensions: &["pm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x49, 0x45, 0x57])],
                },
            }],
        }],
        related_formats: &[],
    },
};
