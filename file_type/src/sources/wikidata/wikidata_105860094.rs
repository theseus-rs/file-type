use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860094: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_094,
        source_type: SourceType::Wikidata,
        name: "Digital Video Interactive video",
        extensions: &["dvi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x44, 0x56, 0x49])],
                },
            }],
        }],
        related_formats: &[],
    },
};
