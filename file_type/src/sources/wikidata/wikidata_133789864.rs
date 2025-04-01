use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_133789864: FileType = FileType {
    file_format: &FileFormat {
        id: 133_789_864,
        source_type: SourceType::Wikidata,
        name: "Portfolio Document",
        extensions: &["fdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x80, 0x47, 0x02, 0x44, 0x02,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
