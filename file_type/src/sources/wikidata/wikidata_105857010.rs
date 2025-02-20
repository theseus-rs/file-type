use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857010: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_010,
        source_type: SourceType::Wikidata,
        name: "Smart Software Graph Definition",
        extensions: &["gdf"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x72, 0x61, 0x70, 0x68, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x20, 0x31, 0x2E, 0x30, 0x30, 0x2E, 0x30, 0x30, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
