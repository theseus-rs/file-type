use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854933: FileType = FileType {
    file_format: &FileFormat {
        id: 105_854_933,
        source_type: SourceType::Wikidata,
        name: "BlackBerry Application Loader",
        extensions: &["alx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x6C, 0x6F, 0x61, 0x64, 0x65, 0x72, 0x20, 0x76, 0x65, 0x72, 0x73,
                        0x69, 0x6F, 0x6E, 0x3D, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
