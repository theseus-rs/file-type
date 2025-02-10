use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857358: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_358,
        source_type: SourceType::Wikidata,
        name: "JTAG Chain File",
        extensions: &["jcf"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x62, 0x65, 0x67, 0x69, 0x6E, 0x3E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
