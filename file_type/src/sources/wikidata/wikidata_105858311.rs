use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858311: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_311,
        source_type: SourceType::Wikidata,
        name: "Live For Speed Engine sound",
        extensions: &["eng"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x46, 0x53, 0x45, 0x4E, 0x47])],
                },
            }],
        }],
        related_formats: &[],
    },
};
