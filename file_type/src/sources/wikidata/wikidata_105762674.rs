use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762674: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_674,
        source_type: SourceType::Wikidata,
        name: "Xbox 360 CON container data file",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x4F, 0x4E, 0x5F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
