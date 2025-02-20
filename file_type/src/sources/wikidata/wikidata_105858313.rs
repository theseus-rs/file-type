use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858313: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_313,
        source_type: SourceType::Wikidata,
        name: "Etherpad document",
        extensions: &["etherpad"],
        media_types: &["text/json"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7B, 0x22, 0x70, 0x61, 0x64, 0x3A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
