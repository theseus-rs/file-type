use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865873: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_873,
        source_type: SourceType::Wikidata,
        name: "Professional Draw page (v3.x)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0F, 0x09, 0x08, 0x08, 0x50, 0x72, 0x6F, 0x66, 0x65, 0x73, 0x73, 0x69,
                        0x6F, 0x6E, 0x61, 0x6C, 0x20, 0x44, 0x72, 0x61, 0x77, 0x20, 0x56, 0x33,
                        0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
