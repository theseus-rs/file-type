use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849801: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_801,
        source_type: SourceType::Wikidata,
        name: "CATIA Part Description (v5 r25)",
        extensions: &["catpart"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x35, 0x5F, 0x43, 0x46, 0x56, 0x32, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
