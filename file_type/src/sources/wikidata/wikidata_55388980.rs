use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_55388980: FileType = FileType {
    file_format: &FileFormat {
        id: 55_388_980,
        source_type: SourceType::Wikidata,
        name: "Gasteiger group CTX file format",
        extensions: &["ctx"],
        media_types: &["chemical/x-ctx"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x20, 0x2F, 0x49, 0x44, 0x45, 0x4E, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
