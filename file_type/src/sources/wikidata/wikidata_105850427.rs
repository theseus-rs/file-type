use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850427: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_427,
        source_type: SourceType::Wikidata,
        name: "Software Design Crypto encrypted data",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x72, 0x79, 0x70, 0x74, 0x6F, 0x48, 0x64, 0x72, 0x42, 0x6C, 0x6B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
