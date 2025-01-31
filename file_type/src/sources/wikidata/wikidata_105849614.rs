use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849614: FileFormat = FileFormat {
    id: 105_849_614,
    puid: "wikidata/105849614",
    name: "BGI (Borland Graphics Interface) font",
    extensions: &["chr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x4B, 0x08, 0x08, 0x42, 0x47, 0x49, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
