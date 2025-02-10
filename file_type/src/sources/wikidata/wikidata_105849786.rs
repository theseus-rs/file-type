use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105849786: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_786,
        source_type: SourceType::Wikidata,
        name: "Cal3D Animation File",
        extensions: &["caf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x41, 0x46, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
