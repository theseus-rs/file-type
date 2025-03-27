use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206894: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_894,
        source_type: SourceType::Wikidata,
        name: "Portable Float Map, color variant",
        extensions: &["pfm", "pnm"],
        media_types: &[
            "image/x-portable-anymap",
            "image/x-portable-floatmap",
            "text/plain",
        ],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
