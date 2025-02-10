use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_29465145: FileType = FileType {
    file_format: &FileFormat {
        id: 29_465_145,
        source_type: SourceType::Wikidata,
        name: "Valve Material Type",
        extensions: &["vmt"],
        media_types: &["application/vnd.valve.source.material"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x22])],
                },
            }],
        }],
        related_formats: &[],
    },
};
