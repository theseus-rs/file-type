use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854083: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_083,
        source_type: SourceType::Wikidata,
        name: "DLT game data archive",
        extensions: &["dlt"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x41, 0x56, 0x45, 0x00, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
