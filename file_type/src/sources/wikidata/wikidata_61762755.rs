use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61762755: FileType = FileType {
    file_format: &FileFormat {
        id: 61_762_755,
        source_type: SourceType::Wikidata,
        name: "Time Zone Information Format",
        extensions: &[],
        media_types: &["application/tzif", "application/tzif-leap"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x5A, 0x69, 0x66])],
                },
            }],
        }],
        related_formats: &[],
    },
};
