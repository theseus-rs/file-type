use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854698: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_698,
        source_type: SourceType::Wikidata,
        name: "Hamarsoft HAP compressed archive (v2.10)",
        extensions: &["hap"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x91, 0x4A, 0x2E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
