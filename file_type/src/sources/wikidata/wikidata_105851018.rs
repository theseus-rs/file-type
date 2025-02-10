use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851018: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_018,
        source_type: SourceType::Wikidata,
        name: "LFToolkit Transformation Rules File",
        extensions: &["trf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x56, 0x61, 0x72, 0x69, 0x61, 0x62, 0x6C, 0x65, 0x73,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
