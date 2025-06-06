use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856550: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_550,
        source_type: SourceType::Wikidata,
        name: "World of Warcraft model Skeleton",
        extensions: &["skel"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4B, 0x4C, 0x31])],
                },
            }],
        }],
        related_formats: &[],
    },
};
