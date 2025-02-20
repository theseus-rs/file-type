use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29167419: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_419,
        source_type: SourceType::Wikidata,
        name: "Internet Adventure Game Engine compiled game",
        extensions: &["iage"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x41, 0x47, 0x45, 0x20, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
