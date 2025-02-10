use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850293: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_293,
        source_type: SourceType::Wikidata,
        name: "CSI MaRKup drawing",
        extensions: &["mrk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x41, 0x54, 0x48, 0x65, 0x61, 0x64, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
