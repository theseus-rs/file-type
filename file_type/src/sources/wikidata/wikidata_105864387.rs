use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864387: FileType = FileType {
    file_format: &FileFormat {
        id: 105_864_387,
        source_type: SourceType::Wikidata,
        name: "Amiga PACK_scn1.library compressed data",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x41, 0x46, 0x49])],
                },
            }],
        }],
        related_formats: &[],
    },
};
