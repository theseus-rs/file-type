use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866506: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_506,
        source_type: SourceType::Wikidata,
        name: "X-CAD Pattern Fill",
        extensions: &["ptf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x43, 0x50, 0x46, 0x00, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
