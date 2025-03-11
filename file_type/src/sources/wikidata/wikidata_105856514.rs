use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856514: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_514,
        source_type: SourceType::Wikidata,
        name: "Wingz Help",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x4E, 0x47, 0x5A, 0x57, 0x5A, 0x48, 0x50,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
