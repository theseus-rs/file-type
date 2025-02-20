use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866077: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_077,
        source_type: SourceType::Wikidata,
        name: "Pretty Good Privacy (PGP) Public Keyring",
        extensions: &["pkr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x99, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
