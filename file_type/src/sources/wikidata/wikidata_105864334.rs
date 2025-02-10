use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105864334: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_334,
        source_type: SourceType::Wikidata,
        name: "The Need for Speed car Performance Specs",
        extensions: &["pbs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x30, 0xFB, 0x00, 0x07, 0x78])],
                },
            }],
        }],
        related_formats: &[],
    },
};
