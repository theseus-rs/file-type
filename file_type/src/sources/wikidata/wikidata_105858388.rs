use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858388: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_388,
        source_type: SourceType::Wikidata,
        name: "EPOC Data (gen)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x50, 0x00, 0x00, 0x10, 0x6D, 0x00, 0x00, 0x10,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
