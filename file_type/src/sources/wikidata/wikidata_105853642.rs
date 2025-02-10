use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853642: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_642,
        source_type: SourceType::Wikidata,
        name: "Maxis UTalk audio",
        extensions: &["utk"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0x54, 0x4D, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
