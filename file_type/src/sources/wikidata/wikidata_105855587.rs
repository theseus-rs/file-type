use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855587: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_587,
        source_type: SourceType::Wikidata,
        name: "Turbo Pascal Overlay",
        extensions: &["ovr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x50, 0x4F, 0x56])],
                },
            }],
        }],
        related_formats: &[],
    },
};
