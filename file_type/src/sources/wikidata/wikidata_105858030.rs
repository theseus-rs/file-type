use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858030: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_030,
        source_type: SourceType::Wikidata,
        name: "Infinity Engine Visual Effect",
        extensions: &["vef"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x45, 0x46, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
