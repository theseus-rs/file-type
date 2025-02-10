use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855336: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_336,
        source_type: SourceType::Wikidata,
        name: "Form Master Form (v4.0)",
        extensions: &["frm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x4D, 0xB9, 0xB2])],
                },
            }],
        }],
        related_formats: &[],
    },
};
