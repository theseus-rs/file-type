use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863307: FileFormat = FileFormat {
    id: 105_863_307,
    puid: "wikidata/105863307",
    name: "Mobipocket eBook Auxiliary data",
    extensions: &["mbp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x50, 0x41, 0x52, 0x4D, 0x4F, 0x42, 0x49, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
