use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105865203: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_203,
        source_type: SourceType::Wikidata,
        name: "InstallShield compiled setup Package",
        extensions: &["pkg"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0xA3])],
                },
            }],
        }],
        related_formats: &[],
    },
};
