use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105863895: FileFormat = FileFormat {
    id: 105_863_895,
    source_type: SourceType::Wikidata,
    name: "METRO 2033 benchmark config",
    extensions: &["mbcfg"],
    media_types: &["text/xml"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x62, 0x65, 0x6E, 0x63, 0x68, 0x20, 0x76, 0x65, 0x72, 0x3D, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
