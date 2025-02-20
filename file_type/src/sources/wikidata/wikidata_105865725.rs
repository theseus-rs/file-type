use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865725: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_725,
        source_type: SourceType::Wikidata,
        name: "KeyTronic Trakball profile",
        extensions: &["pro"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x41, 0x42, 0x45, 0x4C, 0x3A, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
