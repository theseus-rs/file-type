use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105863508: FileFormat = FileFormat {
    id: 105_863_508,
    source_type: SourceType::Wikidata,
    name: "MSX Itinerant Orchestra music format",
    extensions: &["mio"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x69, 0x74, 0x69, 0x6E, 0x65, 0x6C, 0x61, 0x6E, 0x74, 0x20, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
