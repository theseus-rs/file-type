use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854008: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_008,
        source_type: SourceType::Wikidata,
        name: "FMOD Sample Bank format (v3)",
        extensions: &["fsb"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x53, 0x42, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
