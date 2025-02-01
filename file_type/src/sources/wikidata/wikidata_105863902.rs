use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863902: FileFormat = FileFormat {
    id: 105_863_902,
    puid: "wikidata/105863902",
    name: "ANIMagic Map",
    extensions: &["map"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x41, 0x50, 0x31, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
