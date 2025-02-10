use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859589: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_589,
        source_type: SourceType::Wikidata,
        name: "trsvid TV3 video",
        extensions: &["tv3"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x57, 0x50, 0xD6, 0x03])],
                },
            }],
        }],
        related_formats: &[],
    },
};
