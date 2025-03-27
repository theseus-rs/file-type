use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205424: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_424,
        source_type: SourceType::Wikidata,
        name: "Rawzor",
        extensions: &["rwz"],
        media_types: &["image/x-raw-rawzor"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x72, 0x61, 0x77, 0x7A, 0x6F, 0x72])],
                },
            }],
        }],
        related_formats: &[],
    },
};
