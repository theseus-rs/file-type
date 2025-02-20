use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857413: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_413,
        source_type: SourceType::Wikidata,
        name: "JTAG Indirect Configuration",
        extensions: &["jic"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0x49, 0x43, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
