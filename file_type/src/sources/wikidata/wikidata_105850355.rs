use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850355: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_355,
        source_type: SourceType::Wikidata,
        name: "Canon Photo Info file",
        extensions: &["ctg"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3A, 0x5C, 0x44, 0x43, 0x49, 0x4D, 0x5C])],
                },
            }],
        }],
        related_formats: &[],
    },
};
