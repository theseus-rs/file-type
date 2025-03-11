use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851753: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_753,
        source_type: SourceType::Wikidata,
        name: "ClariSSA preferences",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x53, 0x41, 0x50, 0x52, 0x45, 0x46, 0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
