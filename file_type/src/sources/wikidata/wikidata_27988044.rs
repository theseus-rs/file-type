use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27988044: FileType = FileType {
    file_format: &FileFormat {
        id: 27_988_044,
        source_type: SourceType::Wikidata,
        name: "PASTI",
        extensions: &["stx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x53, 0x59, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
