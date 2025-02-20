use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860870: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_870,
        source_type: SourceType::Wikidata,
        name: "RealMedia Secure clip",
        extensions: &["rmx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2E, 0x52, 0x4D, 0x58])],
                },
            }],
        }],
        related_formats: &[],
    },
};
