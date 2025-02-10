use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850438: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_438,
        source_type: SourceType::Wikidata,
        name: "EM400 Configuration",
        extensions: &["cfg"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x63, 0x6F, 0x6D, 0x70, 0x75, 0x74, 0x65, 0x72, 0x20, 0x7B,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
