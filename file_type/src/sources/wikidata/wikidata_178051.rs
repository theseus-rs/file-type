use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_178051: FileType = FileType {
    file_format: &FileFormat {
        id: 178_051,
        source_type: SourceType::Wikidata,
        name: "Portable Network Graphics",
        extensions: &["png"],
        media_types: &["image/png"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
