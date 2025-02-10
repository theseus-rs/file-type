use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859925: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_925,
        source_type: SourceType::Wikidata,
        name: "Bink2 video",
        extensions: &["bik", "bik2", "bk2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4B, 0x42, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
