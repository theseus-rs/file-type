use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855881: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_881,
        source_type: SourceType::Wikidata,
        name: "DeltaCad drawing",
        extensions: &["dc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x43, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
