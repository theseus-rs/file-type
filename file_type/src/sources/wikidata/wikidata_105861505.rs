use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105861505: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_505,
        source_type: SourceType::Wikidata,
        name: "LucasFilm Data - LucasArts game resource",
        extensions: &["lfd"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x4D, 0x41, 0x50, 0x72, 0x65, 0x73, 0x6F, 0x75, 0x72, 0x63, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
