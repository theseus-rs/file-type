use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866130: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_130,
        source_type: SourceType::Wikidata,
        name: "Psion serie 3 Application Alias",
        extensions: &["als"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x41, 0x70, 0x70, 0x41, 0x6C, 0x69, 0x61, 0x73, 0x46, 0x69, 0x6C, 0x65,
                        0x2A, 0x2A, 0x2A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
