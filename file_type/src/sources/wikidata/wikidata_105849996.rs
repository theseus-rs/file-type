use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849996: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_996,
        source_type: SourceType::Wikidata,
        name: "Ch file format (encrypted)",
        extensions: &["ch"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x48, 0x46, 0x45, 0x4E, 0x43, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
