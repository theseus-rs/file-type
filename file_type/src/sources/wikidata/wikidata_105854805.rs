use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854805: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_805,
        source_type: SourceType::Wikidata,
        name: "Caddie Category Sets",
        extensions: &["atc"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x43, 0x61, 0x74, 0x65, 0x67, 0x6F, 0x72, 0x79, 0x53, 0x65, 0x74,
                        0x73, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
