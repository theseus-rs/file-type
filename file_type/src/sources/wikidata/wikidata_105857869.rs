use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857869: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_869,
        source_type: SourceType::Wikidata,
        name: "ICC Animation",
        extensions: &["icc"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x69, 0x63, 0x63, 0x20, 0x61, 0x6E, 0x69, 0x6D, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
