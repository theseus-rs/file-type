use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_137212249: FileType = FileType {
    file_format: &FileFormat {
        id: 137_212_249,
        source_type: SourceType::Wikidata,
        name: "ProDiscover Image",
        extensions: &["eve"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x46, 0x54, 0x20, 0x49, 0x6D, 0x61, 0x67, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
