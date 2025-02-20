use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857139: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_139,
        source_type: SourceType::Wikidata,
        name: "Houdini Apprentice Project",
        extensions: &["hipnc"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x6F, 0x75, 0x4E, 0x43, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
