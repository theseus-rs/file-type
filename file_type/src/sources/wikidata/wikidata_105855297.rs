use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855297: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_297,
        source_type: SourceType::Wikidata,
        name: "StarWriter for MS-DOS Formula",
        extensions: &["frm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4A, 0x4A, 0x01, 0x00, 0x05, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
