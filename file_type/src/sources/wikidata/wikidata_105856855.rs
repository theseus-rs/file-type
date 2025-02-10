use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856855: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_855,
        source_type: SourceType::Wikidata,
        name: "Mind Games - 4 in a Line saved game",
        extensions: &["gam"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x63, 0x66, 0x6F, 0x75, 0x72, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
