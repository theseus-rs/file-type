use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29167848: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_848,
        source_type: SourceType::Wikidata,
        name: "Outlook Express Database",
        extensions: &["dbx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xCF, 0xAD, 0x12, 0xFE])],
                },
            }],
        }],
        related_formats: &[],
    },
};
