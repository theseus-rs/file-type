use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856134: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_134,
        source_type: SourceType::Wikidata,
        name: "GEM Driver Definition",
        extensions: &["ddf"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6D, 0x65, 0x6E, 0x75, 0x6C, 0x61, 0x62, 0x65, 0x6C, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
