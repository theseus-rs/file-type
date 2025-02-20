use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860409: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_409,
        source_type: SourceType::Wikidata,
        name: "IDRISI Raster image Documentation",
        extensions: &["rdc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x66, 0x69, 0x6C, 0x65, 0x20, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x20,
                        0x3A, 0x20, 0x49, 0x44, 0x52, 0x49, 0x53, 0x49, 0x20, 0x52, 0x61, 0x73,
                        0x74, 0x65, 0x72, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
