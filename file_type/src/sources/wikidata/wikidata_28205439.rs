use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205439: FileType = FileType {
    file_format: &FileFormat {
        id: 28_205_439,
        source_type: SourceType::Wikidata,
        name: "Sony ARW",
        extensions: &["arw"],
        media_types: &["image/x-sony-arw"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x49, 0x2A, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
