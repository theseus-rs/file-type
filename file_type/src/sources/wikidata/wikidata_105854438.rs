use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854438: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_438,
        source_type: SourceType::Wikidata,
        name: "Sample DUMP Exchange audio",
        extensions: &["sdx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x44, 0x58, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
