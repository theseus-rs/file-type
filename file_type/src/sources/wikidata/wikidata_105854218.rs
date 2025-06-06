use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854218: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_218,
        source_type: SourceType::Wikidata,
        name: "Advanced Input Recording",
        extensions: &["air"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4F, 0x70, 0x65, 0x6E, 0x41, 0x69, 0x72, 0x21,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
