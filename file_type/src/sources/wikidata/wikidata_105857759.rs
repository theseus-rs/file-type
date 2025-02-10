use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857759: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_759,
        source_type: SourceType::Wikidata,
        name: "Art Icons Pro - IconCollection",
        extensions: &["icc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x43, 0x43, 0x30, 0x01, 0x00, 0x00, 0x00, 0x49, 0x43, 0x4F, 0x4E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
