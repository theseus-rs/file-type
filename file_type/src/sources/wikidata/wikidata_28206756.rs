use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_28206756: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_756,
        source_type: SourceType::Wikidata,
        name: "NIST IHead",
        extensions: &["pct"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x32, 0x38, 0x38, 0x00, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
