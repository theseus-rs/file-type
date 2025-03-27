use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28106155: FileType = FileType {
    file_format: &FileFormat {
        id: 28_106_155,
        source_type: SourceType::Wikidata,
        name: "PIC2",
        extensions: &["p2"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x32, 0x44, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
