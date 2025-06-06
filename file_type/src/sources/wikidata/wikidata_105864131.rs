use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864131: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_131,
        source_type: SourceType::Wikidata,
        name: "mupen64 movie capture",
        extensions: &["m64"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x36, 0x34, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
