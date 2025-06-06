use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862675: FileType = FileType {
    file_format: &FileFormat {
        id: 105_862_675,
        source_type: SourceType::Wikidata,
        name: "Doom 3 MD5 Animation",
        extensions: &["md5anim"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x44, 0x35, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x31,
                        0x30,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
