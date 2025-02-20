use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866382: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_382,
        source_type: SourceType::Wikidata,
        name: "GoBe Productive Document (gen)",
        extensions: &["pve"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x4F, 0x42, 0x45, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
