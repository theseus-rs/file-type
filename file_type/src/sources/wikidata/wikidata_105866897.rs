use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866897: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_897,
        source_type: SourceType::Wikidata,
        name: "PageSetter document",
        extensions: &["doc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x04, 0x03, 0x02, 0x01, 0x00, 0x00, 0x00, 0x01,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
