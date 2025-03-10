use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864397: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_397,
        source_type: SourceType::Wikidata,
        name: "WordWorth preferences",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x57, 0x57, 0x50, 0x72, 0x65, 0x66, 0x73])],
                },
            }],
        }],
        related_formats: &[],
    },
};
