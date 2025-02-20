use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_72960914: FileType = FileType {
    file_format: &FileFormat {
        id: 72_960_914,
        source_type: SourceType::Wikidata,
        name: "Pure Data patch",
        extensions: &["pd"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x23, 0x4E, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
