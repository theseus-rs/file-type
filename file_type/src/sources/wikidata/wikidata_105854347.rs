use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854347: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_347,
        source_type: SourceType::Wikidata,
        name: "FlacPak lossless compressed instrument",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x66, 0x4C, 0x70, 0x4B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
