use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105853026: FileType = FileType {
    file_format: &FileFormat {
        id: 105_853_026,
        source_type: SourceType::Wikidata,
        name: "Note Song",
        extensions: &["sop"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x73, 0x6F, 0x70, 0x65, 0x70, 0x6F, 0x73, 0x00, 0x01, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
