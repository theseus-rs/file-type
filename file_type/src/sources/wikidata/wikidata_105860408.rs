use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860408: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_408,
        source_type: SourceType::Wikidata,
        name: "REKO cardset",
        extensions: &["reko"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x45, 0x4B, 0x4F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
