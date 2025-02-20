use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849851: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_851,
        source_type: SourceType::Wikidata,
        name: "PlayStation RSD Coordinates (v3.0)",
        extensions: &["cod"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x40, 0x43, 0x4F, 0x44, 0x39, 0x37, 0x30, 0x34, 0x30, 0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
