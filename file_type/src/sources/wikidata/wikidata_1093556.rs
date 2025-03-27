use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1093556: FileType = FileType {
    file_format: &FileFormat {
        id: 1_093_556,
        source_type: SourceType::Wikidata,
        name: "eXtensible ARchive format",
        extensions: &["xar"],
        media_types: &["application/x-xar"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x78, 0x61, 0x72, 0x21])],
                },
            }],
        }],
        related_formats: &[],
    },
};
