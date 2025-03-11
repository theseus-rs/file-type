use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852227: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_227,
        source_type: SourceType::Wikidata,
        name: "SpecBAS BASIC program",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x5A, 0x58, 0x42, 0x41, 0x53, 0x49, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
