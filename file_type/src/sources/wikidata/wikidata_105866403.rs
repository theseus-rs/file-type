use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105866403: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_403,
        source_type: SourceType::Wikidata,
        name: "Pro/ENGINEER parts file",
        extensions: &["prt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x55, 0x47, 0x43, 0x3A, 0x32, 0x20, 0x50, 0x41, 0x52, 0x54, 0x20,
                        0x31,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
