use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858110: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_110,
        source_type: SourceType::Wikidata,
        name: "EM400 disk image",
        extensions: &["e4i"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x34, 0x49, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
