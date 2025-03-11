use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856111: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_111,
        source_type: SourceType::Wikidata,
        name: "DUNE Shot",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x45, 0x52, 0x32, 0x53, 0x48, 0x4F, 0x54, 0x53, 0x43, 0x4E, 0x46,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
