use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858504: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_504,
        source_type: SourceType::Wikidata,
        name: "Encrypted Multi-Picture Object bitmap",
        extensions: &["empo"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x4D, 0x50, 0x4F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
