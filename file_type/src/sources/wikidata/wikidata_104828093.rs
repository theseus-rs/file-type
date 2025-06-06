use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_104828093: FileType = FileType {
    file_format: &FileFormat {
        id: 104_828_093,
        source_type: SourceType::Wikidata,
        name: "DiamondWare Digitized",
        extensions: &["dwd"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x69, 0x61, 0x6D, 0x6F, 0x6E, 0x64, 0x57, 0x61, 0x72, 0x65, 0x20,
                        0x44, 0x69, 0x67, 0x69, 0x74, 0x69, 0x7A, 0x65, 0x64,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
