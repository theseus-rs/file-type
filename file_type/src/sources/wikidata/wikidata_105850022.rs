use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850022: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_022,
        source_type: SourceType::Wikidata,
        name: "CISO Compressed ISO CD image",
        extensions: &["ciso", "cso", "wbi"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x49, 0x53, 0x4F])],
                },
            }],
        }],
        related_formats: &[],
    },
};
