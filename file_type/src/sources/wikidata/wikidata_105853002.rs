use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105853002: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_002,
        source_type: SourceType::Wikidata,
        name: "Datalink SoundScape",
        extensions: &["spc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25, 0x04, 0x19, 0x69])],
                },
            }],
        }],
        related_formats: &[],
    },
};
