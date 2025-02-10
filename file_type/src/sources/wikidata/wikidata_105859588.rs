use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859588: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_588,
        source_type: SourceType::Wikidata,
        name: "Sierra Video and Music Data video",
        extensions: &["vmd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2E, 0x03, 0x00, 0x00, 0x01, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
