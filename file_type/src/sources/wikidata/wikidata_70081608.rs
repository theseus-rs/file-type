use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_70081608: FileType = FileType {
    file_format: &FileFormat {
        id: 70_081_608,
        source_type: SourceType::Wikidata,
        name: "Oracle FM instruments Library format",
        extensions: &["fml"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4F, 0x52, 0x41, 0x43, 0x4C, 0x45, 0x00, 0x01, 0x04,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
