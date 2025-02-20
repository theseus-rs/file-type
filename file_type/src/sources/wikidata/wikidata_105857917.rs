use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857917: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_917,
        source_type: SourceType::Wikidata,
        name: "SL9821 Hard disk image",
        extensions: &["slh"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x44, 0x49, 0x4D])],
                },
            }],
        }],
        related_formats: &[],
    },
};
