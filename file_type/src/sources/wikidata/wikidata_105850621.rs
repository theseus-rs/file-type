use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850621: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_621,
        source_type: SourceType::Wikidata,
        name: "Multibit Blockchain Checkpoints data",
        extensions: &["checkpoints"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x48, 0x45, 0x43, 0x4B, 0x50, 0x4F, 0x49, 0x4E, 0x54, 0x53, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
