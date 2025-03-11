use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866027: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_027,
        source_type: SourceType::Wikidata,
        name: "Protext document (v6.x)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00, 0x50, 0x52, 0x4F, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
