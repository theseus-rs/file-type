use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28049588: FileType = FileType {
    file_format: &FileFormat {
        id: 28_049_588,
        source_type: SourceType::Wikidata,
        name: "Tiny Stuff, low resolution",
        extensions: &["tn1", "tny"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x07, 0x77])],
                },
            }],
        }],
        related_formats: &[],
    },
};
