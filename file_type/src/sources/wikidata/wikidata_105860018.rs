use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860018: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_018,
        source_type: SourceType::Wikidata,
        name: "V-Ray Material (XML)",
        extensions: &["vrmat"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x76, 0x69, 0x73, 0x6D, 0x61, 0x74, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
