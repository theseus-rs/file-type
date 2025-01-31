use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863489: FileFormat = FileFormat {
    id: 105_863_489,
    puid: "wikidata/105863489",
    name: "Bob's AdLib Music module",
    extensions: &["bam"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x42, 0x4D, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
