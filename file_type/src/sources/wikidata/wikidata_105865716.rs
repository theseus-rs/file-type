use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865716: FileType = FileType {
    file_format: &FileFormat {
        id: 105_865_716,
        source_type: SourceType::Wikidata,
        name: "Promizer 4.0 song/module",
        extensions: &[],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x4D, 0x34, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
