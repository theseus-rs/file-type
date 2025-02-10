use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_124670600: FileType = FileType {
    file_format: &FileFormat {
        id: 124_670_600,
        source_type: SourceType::Wikidata,
        name: "PCX, version 0",
        extensions: &["pcx"],
        media_types: &["image/x-pcx"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
