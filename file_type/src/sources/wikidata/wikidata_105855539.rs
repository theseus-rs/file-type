use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855539: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_539,
        source_type: SourceType::Wikidata,
        name: "OpenStreetMap Binary map data",
        extensions: &["osb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4F, 0x53, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
