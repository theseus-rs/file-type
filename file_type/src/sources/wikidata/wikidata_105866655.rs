use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866655: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_655,
        source_type: SourceType::Wikidata,
        name: "Newton Package (NOS v1.x)",
        extensions: &["pkg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x70, 0x61, 0x63, 0x6B, 0x61, 0x67, 0x65, 0x30, 0x78, 0x78, 0x78, 0x78,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
