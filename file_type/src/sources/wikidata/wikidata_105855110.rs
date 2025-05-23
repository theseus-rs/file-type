use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855110: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_110,
        source_type: SourceType::Wikidata,
        name: "THOR compressed data",
        extensions: &["thr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x48, 0x52])],
                },
            }],
        }],
        related_formats: &[],
    },
};
