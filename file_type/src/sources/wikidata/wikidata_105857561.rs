use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857561: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_561,
        source_type: SourceType::Wikidata,
        name: "Infinity Engine Creature (generic)",
        extensions: &["cre"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x52, 0x45, 0x20, 0x56])],
                },
            }],
        }],
        related_formats: &[],
    },
};
