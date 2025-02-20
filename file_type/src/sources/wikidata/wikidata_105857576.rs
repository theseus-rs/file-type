use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857576: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_576,
        source_type: SourceType::Wikidata,
        name: "InstallShield Script",
        extensions: &["ins"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xB8, 0xC9, 0x0C, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
