use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852937: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_937,
        source_type: SourceType::Wikidata,
        name: "ID Software Sprite format",
        extensions: &["spr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x44, 0x53, 0x50])],
                },
            }],
        }],
        related_formats: &[],
    },
};
