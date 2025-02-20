use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117382180: FileType = FileType {
    file_format: &FileFormat {
        id: 117_382_180,
        source_type: SourceType::Wikidata,
        name: "Extended Notation Format",
        extensions: &["enf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x4E, 0x46, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
