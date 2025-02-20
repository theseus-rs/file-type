use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849951: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_951,
        source_type: SourceType::Wikidata,
        name: "Doom Configuration",
        extensions: &["cfg"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6D, 0x6F, 0x75, 0x73, 0x65, 0x5F, 0x73, 0x65, 0x6E, 0x73, 0x69, 0x74,
                        0x69, 0x76, 0x69, 0x74, 0x79,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
