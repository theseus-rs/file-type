use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855208: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_208,
        source_type: SourceType::Wikidata,
        name: "Frodo Preferences",
        extensions: &["fpr"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4E, 0x6F, 0x72, 0x6D, 0x61, 0x6C, 0x43, 0x79, 0x63, 0x6C, 0x65, 0x73,
                        0x20, 0x3D, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
