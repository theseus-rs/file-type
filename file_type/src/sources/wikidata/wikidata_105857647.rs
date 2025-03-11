use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857647: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_647,
        source_type: SourceType::Wikidata,
        name: "InstallShield Setup Stream",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x53, 0x53, 0x65, 0x74, 0x75, 0x70, 0x53, 0x74, 0x72, 0x65, 0x61,
                        0x6D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
