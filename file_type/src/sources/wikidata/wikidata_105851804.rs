use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851804: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_804,
        source_type: SourceType::Wikidata,
        name: "Photono-Software Stealther Skin",
        extensions: &["skn"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x78, 0x9C, 0xEC, 0xBD])],
                },
            }],
        }],
        related_formats: &[],
    },
};
