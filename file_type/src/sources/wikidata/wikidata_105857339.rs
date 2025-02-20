use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857339: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_339,
        source_type: SourceType::Wikidata,
        name: "Embedded JCL debug info",
        extensions: &["jdbg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0x44, 0x42, 0x47])],
                },
            }],
        }],
        related_formats: &[],
    },
};
