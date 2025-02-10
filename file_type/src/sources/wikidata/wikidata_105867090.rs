use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105867090: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_090,
        source_type: SourceType::Wikidata,
        name: "Folio Views Infobase",
        extensions: &["nfo"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x28, 0x63, 0x29, 0x20, 0x46, 0x6F, 0x6C, 0x69, 0x6F, 0x20, 0x43, 0x6F,
                        0x72, 0x70, 0x6F, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x20, 0x31, 0x39,
                        0x39, 0x31, 0x2D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
