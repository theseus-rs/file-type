use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105857130: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_130,
        source_type: SourceType::Wikidata,
        name: "Print / Instant Artist for DOS Graphics",
        extensions: &["gfx"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xF0, 0x0D, 0x03, 0x00])],
                },
            }],
        }],
        related_formats: &[],
    },
};
