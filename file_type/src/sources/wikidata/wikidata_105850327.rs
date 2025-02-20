use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850327: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_327,
        source_type: SourceType::Wikidata,
        name: "Biosym archive",
        extensions: &["car"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x21, 0x42, 0x49, 0x4F, 0x53, 0x59, 0x4D, 0x20, 0x61, 0x72, 0x63, 0x68,
                        0x69, 0x76, 0x65, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
