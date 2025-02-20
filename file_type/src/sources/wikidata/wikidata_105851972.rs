use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851972: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_972,
        source_type: SourceType::Wikidata,
        name: "Psion Archive Screen",
        extensions: &["scn"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x64, 0x62, 0x73, 0x30])],
                },
            }],
        }],
        related_formats: &[],
    },
};
