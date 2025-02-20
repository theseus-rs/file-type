use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857881: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_881,
        source_type: SourceType::Wikidata,
        name: "MySQL Dictionary File (ISAM)",
        extensions: &["frm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0x01, 0x07, 0x07])],
                },
            }],
        }],
        related_formats: &[],
    },
};
