use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105856750: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_750,
        source_type: SourceType::Wikidata,
        name: "Universal Go Format",
        extensions: &["ugf", "ugi"],
        media_types: &["text/ini"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
