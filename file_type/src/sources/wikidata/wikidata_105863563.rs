use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863563: FileFormat = FileFormat {
    id: 105_863_563,
    source_type: SourceType::Wikidata,
    name: "Magnetic Graphics",
    extensions: &["gfx"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x61, 0x50, 0x69])],
            },
        }],
    }],
    related_formats: &[],
};
