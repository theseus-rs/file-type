use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862948: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_948,
        source_type: SourceType::Wikidata,
        name: "Garmin MapSource data",
        extensions: &["mps"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x73, 0x52, 0x63, 0x64])],
                },
            }],
        }],
        related_formats: &[],
    },
};
