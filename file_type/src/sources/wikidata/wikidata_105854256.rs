use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854256: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_256,
        source_type: SourceType::Wikidata,
        name: "Alpha Four data (generic)",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x34])],
                },
            }],
        }],
        related_formats: &[],
    },
};
