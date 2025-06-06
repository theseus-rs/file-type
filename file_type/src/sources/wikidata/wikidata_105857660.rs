use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857660: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_660,
        source_type: SourceType::Wikidata,
        name: "divANS Intermediate Representation",
        extensions: &["ir"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x77, 0x69, 0x6E, 0x64, 0x6F, 0x77, 0x20])],
                },
            }],
        }],
        related_formats: &[],
    },
};
