use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762686: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_686,
        source_type: SourceType::Wikidata,
        name: "Xbox 360 PIRS container data file",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x49, 0x52, 0x53])],
                },
            }],
        }],
        related_formats: &[],
    },
};
