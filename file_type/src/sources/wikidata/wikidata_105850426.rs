use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850426: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_426,
        source_type: SourceType::Wikidata,
        name: "Hydrogen Configuration",
        extensions: &["conf"],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x68, 0x79, 0x64, 0x72, 0x6F, 0x67, 0x65, 0x6E, 0x5F, 0x70, 0x72,
                        0x65, 0x66, 0x65, 0x72, 0x65, 0x6E, 0x63, 0x65, 0x73, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
