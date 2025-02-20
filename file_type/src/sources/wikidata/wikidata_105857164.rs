use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857164: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_164,
        source_type: SourceType::Wikidata,
        name: "Microsoft Hearts Saved game",
        extensions: &["heartssave-ms"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x47, 0x4D, 0x48, 0x01, 0x00, 0x00, 0x00, 0x28, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
