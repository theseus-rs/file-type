use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_44071424: FileType = FileType {
    file_format: &FileFormat {
        id: 44_071_424,
        source_type: SourceType::Wikidata,
        name: "STATA Data File Format, version 117",
        extensions: &["dta"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x73, 0x74, 0x61, 0x74, 0x61, 0x5F, 0x64, 0x74, 0x61, 0x3E, 0x3C,
                        0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x3E, 0x3C, 0x72, 0x65, 0x6C, 0x65,
                        0x61, 0x73, 0x65, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
