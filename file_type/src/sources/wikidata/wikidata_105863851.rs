use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863851: FileFormat = FileFormat {
    id: 105_863_851,
    source_type: SourceType::Wikidata,
    name: "MENG game data archive (v4)",
    extensions: &["mfs"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x45, 0x4E, 0x47, 0x78, 0x56, 0x34, 0x18, 0x50, 0x41, 0x43, 0x4B, 0x31,
                    0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
