use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851038: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_038,
        source_type: SourceType::Wikidata,
        name: "Vortex Tracker II workspace",
        extensions: &["txt"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x4D, 0x6F, 0x64, 0x75, 0x6C, 0x65, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
