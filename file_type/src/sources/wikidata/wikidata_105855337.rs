use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855337: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_337,
        source_type: SourceType::Wikidata,
        name: "Formula document (v1.0)",
        extensions: &["frm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x02, 0x10, 0x64, 0x84, 0x00, 0x01, 0x01, 0x00, 0x01,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
