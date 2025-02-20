use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856914: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_914,
        source_type: SourceType::Wikidata,
        name: "Gens movie capture",
        extensions: &["gmv"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x47, 0x65, 0x6E, 0x73, 0x20, 0x4D, 0x6F, 0x76, 0x69, 0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
