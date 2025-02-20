use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860081: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_081,
        source_type: SourceType::Wikidata,
        name: "Viacom New Media graphics",
        extensions: &["000", "vnm"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x4E, 0x4D, 0x1A])],
                },
            }],
        }],
        related_formats: &[],
    },
};
