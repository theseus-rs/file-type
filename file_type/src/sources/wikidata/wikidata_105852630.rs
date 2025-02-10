use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852630: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_630,
        source_type: SourceType::Wikidata,
        name: "gEDA Symbol",
        extensions: &["sym"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x76, 0x20, 0x32, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
