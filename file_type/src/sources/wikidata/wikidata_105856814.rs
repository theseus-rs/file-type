use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856814: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_814,
        source_type: SourceType::Wikidata,
        name: "Arts and Letters Graphics file",
        extensions: &["ged"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x26, 0x4C, 0x2D, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
