use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858076: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_076,
        source_type: SourceType::Wikidata,
        name: "IBrowse Global Cache",
        extensions: &[],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x24, 0x56, 0x45, 0x52, 0x3A, 0x20, 0x49, 0x42, 0x72, 0x6F, 0x77, 0x73,
                        0x65, 0x47, 0x6C, 0x6F, 0x62, 0x61, 0x6C, 0x43, 0x61, 0x63, 0x68, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
