use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_87066063: FileType = FileType {
    file_format: &FileFormat {
        id: 87_066_063,
        source_type: SourceType::Wikidata,
        name: "LEADTools Lead 1Bit Compressed Image",
        extensions: &["cmp"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x54, 0x52, 0x49])],
                },
            }],
        }],
        related_formats: &[],
    },
};
