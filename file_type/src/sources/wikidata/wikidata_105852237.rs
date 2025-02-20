use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852237: FileType = FileType {
    file_format: &FileFormat {
        id: 105_852_237,
        source_type: SourceType::Wikidata,
        name: "Sasami Script subtitles",
        extensions: &["s2k"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3B, 0x45, 0x6E, 0x76, 0x2E, 0x4D, 0x6F, 0x76, 0x69, 0x65, 0x2E, 0x57,
                        0x69, 0x64, 0x74, 0x68, 0x3D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
