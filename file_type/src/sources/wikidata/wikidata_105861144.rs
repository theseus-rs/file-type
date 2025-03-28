use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861144: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_144,
        source_type: SourceType::Wikidata,
        name: "Liko-12 disk",
        extensions: &["lk12"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x4B, 0x31, 0x32, 0x3B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
