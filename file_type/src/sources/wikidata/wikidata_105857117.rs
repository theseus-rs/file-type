use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857117: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_117,
        source_type: SourceType::Wikidata,
        name: "GIMS Graphical Text data",
        extensions: &["gxt"],
        media_types: &["text/ini"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5B, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x49, 0x6E, 0x66, 0x6F, 0x5D,
                        0x0D, 0x0A, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x5F, 0x76, 0x65, 0x72,
                        0x3D, 0x31, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
