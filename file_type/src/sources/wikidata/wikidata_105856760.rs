use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856760: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_760,
        source_type: SourceType::Wikidata,
        name: "Windows Explorer UIFILE",
        extensions: &["uifile"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3C, 0x73, 0x74, 0x79, 0x6C, 0x65])],
                },
            }],
        }],
        related_formats: &[],
    },
};
