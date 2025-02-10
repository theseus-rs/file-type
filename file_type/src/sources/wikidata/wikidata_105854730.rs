use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854730: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_730,
        source_type: SourceType::Wikidata,
        name: "FMOD Sample Bank format (v5)",
        extensions: &["fsb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x53, 0x42, 0x35])],
                },
            }],
        }],
        related_formats: &[],
    },
};
