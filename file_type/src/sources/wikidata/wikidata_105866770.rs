use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866770: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_770,
        source_type: SourceType::Wikidata,
        name: "Palm ThoughtManager",
        extensions: &["pdb"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x4F, 0x75, 0x74, 0x6C, 0x54, 0x46, 0x73, 0x74,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
