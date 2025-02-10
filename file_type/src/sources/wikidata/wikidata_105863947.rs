use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105863947: FileType = FileType {
    file_format: &FileFormat {
        id: 105_863_947,
        source_type: SourceType::Wikidata,
        name: "Mizar article (with rem)",
        extensions: &["miz"],
        media_types: &["text/mizar"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3A, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
