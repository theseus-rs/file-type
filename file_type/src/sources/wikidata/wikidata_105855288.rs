use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855288: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_288,
        source_type: SourceType::Wikidata,
        name: "Face The Music Effect",
        extensions: &["ef"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x46, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
