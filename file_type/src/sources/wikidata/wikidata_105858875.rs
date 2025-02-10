use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105858875: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_875,
        source_type: SourceType::Wikidata,
        name: "FaceSaver bitmap",
        extensions: &["fac", "face"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x69, 0x72, 0x73, 0x74, 0x4E, 0x61, 0x6D, 0x65, 0x3A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
