use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855228: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_228,
        source_type: SourceType::Wikidata,
        name: "FastDir-like quick directory lookup data",
        extensions: &["fd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x48, 0x43, 0x48])],
                },
            }],
        }],
        related_formats: &[],
    },
};
