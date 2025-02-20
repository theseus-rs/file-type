use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852072: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_072,
        source_type: SourceType::Wikidata,
        name: "Descent Game Save",
        extensions: &["sg0"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x47, 0x53, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
