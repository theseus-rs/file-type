use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28770288: FileType = FileType {
    file_format: &FileFormat {
        id: 28_770_288,
        source_type: SourceType::Wikidata,
        name: "LBR",
        extensions: &["lbr"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x57, 0x42, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
