use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850009: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_009,
        source_type: SourceType::Wikidata,
        name: "Cap'n Proto schema",
        extensions: &["capnp"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x40, 0x30, 0x78])],
                },
            }],
        }],
        related_formats: &[],
    },
};
