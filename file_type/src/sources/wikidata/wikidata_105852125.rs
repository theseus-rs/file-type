use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852125: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_125,
        source_type: SourceType::Wikidata,
        name: "Snzip compressed (snzip format)",
        extensions: &["snz"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x4E, 0x5A, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
