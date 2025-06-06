use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861653: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_653,
        source_type: SourceType::Wikidata,
        name: "CP/M-86 library",
        extensions: &["l86"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xA4, 0x07, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
