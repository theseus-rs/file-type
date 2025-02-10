use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_3928266: FileType = FileType {
    file_format: &FileFormat {
        id: 3_928_266,
        source_type: SourceType::Wikidata,
        name: "RF64",
        extensions: &["rf64", "wav"],
        media_types: &["audio/vnd.wave"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x46, 0x36, 0x34, 0xFF, 0xFF, 0xFF, 0xFF, 0x57, 0x41, 0x56, 0x45,
                        0x64, 0x73, 0x36, 0x34,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
