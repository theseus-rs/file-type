use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851910: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_910,
        source_type: SourceType::Wikidata,
        name: "Pretty Good Privacy (PGP) Private/Secret Keyring",
        extensions: &["gpg", "pgp", "skr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x95, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
