use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857727: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_727,
        source_type: SourceType::Wikidata,
        name: "CBASIC Intermediate code",
        extensions: &["int"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x02, 0x31, 0x24, 0x2A, 0x00, 0x08, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
