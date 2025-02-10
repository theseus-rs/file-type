use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_27979374: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_374,
        source_type: SourceType::Wikidata,
        name: "Spruce subtitle format",
        extensions: &["stl"],
        media_types: &["text/plain"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x24, 0x46, 0x6F, 0x6E, 0x74, 0x4E, 0x61, 0x6D, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
