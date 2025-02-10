use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867537: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_537,
        source_type: SourceType::Wikidata,
        name: "Open Minecraft Note Block Studio Song (v4)",
        extensions: &["nbs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00, 0x04, 0x10])],
                },
            }],
        }],
        related_formats: &[],
    },
};
