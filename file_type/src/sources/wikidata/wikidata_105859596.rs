use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859596: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_596,
        source_type: SourceType::Wikidata,
        name: "Nancy Codec video",
        extensions: &["noa"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xD2, 0x4E, 0x4F, 0x41, 0x0F, 0x0C, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
