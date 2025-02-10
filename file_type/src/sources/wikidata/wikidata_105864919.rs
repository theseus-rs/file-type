use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864919: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_919,
        source_type: SourceType::Wikidata,
        name: "Plain Old Documentation format",
        extensions: &["pod"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3D, 0x68, 0x65, 0x61, 0x64, 0x31, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
