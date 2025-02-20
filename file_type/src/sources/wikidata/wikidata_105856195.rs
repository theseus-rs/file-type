use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856195: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_195,
        source_type: SourceType::Wikidata,
        name: "Adobe Dimensions geometry data (v1.0)",
        extensions: &["dim"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x0D, 0x0A, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
