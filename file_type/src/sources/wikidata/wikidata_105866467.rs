use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866467: FileType = FileType {
    file_format: &FileFormat {
        id: 105_866_467,
        source_type: SourceType::Wikidata,
        name: "WinAmp/SHOUTcast PlayList",
        extensions: &["pls"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x70, 0x6C, 0x61, 0x79, 0x6C, 0x69, 0x73, 0x74, 0x5D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
