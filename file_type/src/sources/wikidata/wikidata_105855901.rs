use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855901: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_901,
        source_type: SourceType::Wikidata,
        name: "TS Online Dialing Directory",
        extensions: &["dcd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x43, 0x44, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
