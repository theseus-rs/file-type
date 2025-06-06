use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206946: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_946,
        source_type: SourceType::Wikidata,
        name: "PhotoChrome",
        extensions: &["pcs"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x40, 0x00, 0xC8])],
                },
            }],
        }],
        related_formats: &[],
    },
};
