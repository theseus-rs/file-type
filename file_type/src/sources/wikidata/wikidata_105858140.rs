use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858140: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_140,
        source_type: SourceType::Wikidata,
        name: "Bochs growing disk image",
        extensions: &["img"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x42, 0x6F, 0x63, 0x68, 0x73, 0x20, 0x56, 0x69, 0x72, 0x74, 0x75, 0x61,
                        0x6C, 0x20, 0x48, 0x44, 0x20, 0x49, 0x6D, 0x61, 0x67, 0x65, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x52, 0x65, 0x64, 0x6F,
                        0x6C, 0x6F, 0x67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x47, 0x72, 0x6F, 0x77, 0x69, 0x6E, 0x67,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
