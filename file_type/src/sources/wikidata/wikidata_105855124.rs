use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105855124: FileType = FileType {
    file_format: &FileFormat {
        id: 105_855_124,
        source_type: SourceType::Wikidata,
        name: "PaperPort slide show",
        extensions: &["fss"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x53, 0x42, 0x4F, 0x42])],
                },
            }],
        }],
        related_formats: &[],
    },
};
