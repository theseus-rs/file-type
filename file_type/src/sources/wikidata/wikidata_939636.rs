use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_939636: FileType = FileType {
    file_format: &FileFormat {
        id: 939_636,
        source_type: SourceType::Wikidata,
        name: "CorelDRAW Document",
        extensions: &["cdr", "cdt", "cdx", "cpx", "csl", "pat"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x52, 0x49, 0x46, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
