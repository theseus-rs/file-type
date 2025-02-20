use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967540: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_540,
        source_type: SourceType::Wikidata,
        name: "Cyber Paint Sequence",
        extensions: &["seq"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0xDB, 0x00, 0x00, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
