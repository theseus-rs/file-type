use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856121: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_121,
        source_type: SourceType::Wikidata,
        name: "Device Tree Source (with rem)",
        extensions: &["dts"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x2F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
