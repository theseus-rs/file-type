use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854027: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_027,
        source_type: SourceType::Wikidata,
        name: "Vue D'Esprit 4 Atmosphere Preset",
        extensions: &["atm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
