use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857256: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_256,
        source_type: SourceType::Wikidata,
        name: "HomeBrew Level",
        extensions: &["hle"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x48, 0x6F, 0x6D, 0x65, 0x42, 0x72, 0x65, 0x77, 0x20, 0x4C, 0x65, 0x76,
                        0x65, 0x6C, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
