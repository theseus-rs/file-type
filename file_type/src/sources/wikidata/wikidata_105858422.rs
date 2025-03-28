use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858422: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_422,
        source_type: SourceType::Wikidata,
        name: "Resident Evil player model data",
        extensions: &["emw"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x64, 0x00, 0xB0, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
