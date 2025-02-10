use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851932: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_932,
        source_type: SourceType::Wikidata,
        name: "FMS Scenery",
        extensions: &["scn"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4B, 0x59, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
