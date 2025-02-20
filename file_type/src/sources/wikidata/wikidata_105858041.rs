use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858041: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_041,
        source_type: SourceType::Wikidata,
        name: "Raven Software compiled script",
        extensions: &["ibi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x42, 0x49, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
