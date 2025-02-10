use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853146: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_146,
        source_type: SourceType::Wikidata,
        name: "Moebius Sound Library",
        extensions: &["slb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4C, 0x42, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
