use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866201: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_201,
        source_type: SourceType::Wikidata,
        name: "PFS:Graph chart",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x67, 0x72, 0x61, 0x66])],
                },
            }],
        }],
        related_formats: &[],
    },
};
