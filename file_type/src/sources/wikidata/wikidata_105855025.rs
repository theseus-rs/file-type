use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855025: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_025,
        source_type: SourceType::Wikidata,
        name: "SGA game data archive",
        extensions: &["sga"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5F, 0x41, 0x52, 0x43, 0x48, 0x49, 0x56, 0x45,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
