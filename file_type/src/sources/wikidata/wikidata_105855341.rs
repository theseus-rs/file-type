use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855341: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_341,
        source_type: SourceType::Wikidata,
        name: "XPS FixedPage object",
        extensions: &["fpage"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x46, 0x69, 0x78, 0x65, 0x64, 0x50, 0x61, 0x67, 0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
