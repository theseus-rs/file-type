use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864504: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_504,
        source_type: SourceType::Wikidata,
        name: "PROGRESS Procedure Library (v11)",
        extensions: &["pl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xD7, 0x0B, 0x07])],
                },
            }],
        }],
        related_formats: &[],
    },
};
