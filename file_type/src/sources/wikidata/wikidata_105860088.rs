use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860088: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_088,
        source_type: SourceType::Wikidata,
        name: "V9990 font format",
        extensions: &["vff"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x46, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
