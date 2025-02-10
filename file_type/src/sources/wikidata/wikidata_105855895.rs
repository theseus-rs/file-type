use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855895: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_895,
        source_type: SourceType::Wikidata,
        name: "Twisted Metal 2 (PC) texture",
        extensions: &["dpc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x44, 0x43, 0x50, 0x4D, 0x43])],
                },
            }],
        }],
        related_formats: &[],
    },
};
