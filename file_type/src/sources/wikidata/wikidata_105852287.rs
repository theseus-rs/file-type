use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852287: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_287,
        source_type: SourceType::Wikidata,
        name: "Sc2gears Mouse Print Data",
        extensions: &["smpd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4D, 0x50, 0x44])],
                },
            }],
        }],
        related_formats: &[],
    },
};
