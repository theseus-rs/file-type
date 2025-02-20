use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860016: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_016,
        source_type: SourceType::Wikidata,
        name: "Electronic Arts TGV video",
        extensions: &["tgv"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6B, 0x56, 0x47, 0x54])],
                },
            }],
        }],
        related_formats: &[],
    },
};
