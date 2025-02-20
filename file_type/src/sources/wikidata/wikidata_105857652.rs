use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857652: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_652,
        source_type: SourceType::Wikidata,
        name: "ZX Wafadrive Wafer image",
        extensions: &["wdr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5A, 0x58, 0x57, 0x61, 0x66, 0x65, 0x72, 0x21,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
