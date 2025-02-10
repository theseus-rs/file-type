use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857242: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_242,
        source_type: SourceType::Wikidata,
        name: "Altera Hierarchy Interconnect File",
        extensions: &["hif"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x49, 0x46, 0x30, 0x30, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
