use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852295: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_295,
        source_type: SourceType::Wikidata,
        name: "Software Ideas Modeler Project (encrypted)",
        extensions: &["simp"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x53, 0x49, 0x4D, 0x3A, 0x45, 0x4E, 0x43, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
