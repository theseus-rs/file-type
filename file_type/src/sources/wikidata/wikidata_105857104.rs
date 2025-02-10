use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857104: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_104,
        source_type: SourceType::Wikidata,
        name: "GNU Privacy Guard public keyring (generic)",
        extensions: &["gpg"],
        media_types: &["application/pgp-keys"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x99])],
                },
            }],
        }],
        related_formats: &[],
    },
};
