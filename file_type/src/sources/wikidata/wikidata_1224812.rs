use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_1224812: FileType = FileType {
    file_format: &FileFormat {
        id: 1_224_812,
        source_type: SourceType::Wikidata,
        name: "Digital Speech Standard",
        extensions: &["ds2", "dss"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x64, 0x73, 0x73])],
                },
            }],
        }],
        related_formats: &[],
    },
};
