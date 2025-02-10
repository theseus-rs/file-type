use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850289: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_289,
        source_type: SourceType::Wikidata,
        name: "Carrara Environment",
        extensions: &["car"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x33, 0x44, 0x43, 0x20, 0x7B])],
                },
            }],
        }],
        related_formats: &[],
    },
};
