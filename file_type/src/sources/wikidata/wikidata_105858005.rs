use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858005: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_005,
        source_type: SourceType::Wikidata,
        name: "Indigo Image",
        extensions: &["igi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7D, 0x70, 0xF8, 0x03])],
                },
            }],
        }],
        related_formats: &[],
    },
};
