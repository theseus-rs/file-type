use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_913946: FileType = FileType {
    file_format: &FileFormat {
        id: 913_946,
        source_type: SourceType::Wikidata,
        name: "NES Sound Format",
        extensions: &["nsf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4E, 0x45, 0x53, 0x4D, 0x1A, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
