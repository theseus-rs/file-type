use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105852129: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_129,
        source_type: SourceType::Wikidata,
        name: "GFA Raytrace compressed Animation (low-res)",
        extensions: &["sal"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x73, 0x61, 0x6C, 0x0D, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
