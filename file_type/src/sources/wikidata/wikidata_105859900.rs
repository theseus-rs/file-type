use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859900: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_900,
        source_type: SourceType::Wikidata,
        name: "VBIN container",
        extensions: &["vbin"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x42, 0x49, 0x4E])],
                },
            }],
        }],
        related_formats: &[],
    },
};
