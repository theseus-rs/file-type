use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857745: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_745,
        source_type: SourceType::Wikidata,
        name: "HXTape Image",
        extensions: &["hxi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x58, 0x49, 0x0A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
