use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27960138: FileType = FileType {
    file_format: &FileFormat {
        id: 27_960_138,
        source_type: SourceType::Wikidata,
        name: "SPPACK",
        extensions: &["d"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x40, 0xC3, 0xFC, 0x0E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
