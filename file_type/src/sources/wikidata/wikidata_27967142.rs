use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967142: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_142,
        source_type: SourceType::Wikidata,
        name: "Digitrakker sample",
        extensions: &["spl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x53, 0x50, 0x4C, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
