use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860232: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_232,
        source_type: SourceType::Wikidata,
        name: "InstallShield Script for Windows Registry",
        extensions: &["rgs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x4B, 0x43, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
