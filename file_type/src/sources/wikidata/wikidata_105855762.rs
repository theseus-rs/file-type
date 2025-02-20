use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855762: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_762,
        source_type: SourceType::Wikidata,
        name: "DFF format (v2.0, LE)",
        extensions: &["dff"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x32, 0x46, 0x44, 0x25])],
                },
            }],
        }],
        related_formats: &[],
    },
};
