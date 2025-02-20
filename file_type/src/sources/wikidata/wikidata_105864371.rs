use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864371: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_371,
        source_type: SourceType::Wikidata,
        name: "Formatta Portable Form File",
        extensions: &["pff"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x46, 0x46, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
