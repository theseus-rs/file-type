use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854737: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_737,
        source_type: SourceType::Wikidata,
        name: "AMOS PowerPacker bank",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x50, 0x62, 0x6B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
