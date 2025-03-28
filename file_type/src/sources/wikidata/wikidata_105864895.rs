use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864895: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_895,
        source_type: SourceType::Wikidata,
        name: "Password Safe database",
        extensions: &["psafe3"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x57, 0x53, 0x33])],
                },
            }],
        }],
        related_formats: &[],
    },
};
