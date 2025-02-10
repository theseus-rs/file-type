use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857395: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_395,
        source_type: SourceType::Wikidata,
        name: "JavaScript Bean file",
        extensions: &["jsb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x4A, 0x53, 0x42, 0x3E, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
