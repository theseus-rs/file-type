use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_81413027: FileFormat = FileFormat {
    id: 81_413_027,
    puid: "wikidata/81413027",
    name: "Capella sheet data file",
    extensions: &["cap"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x63, 0x61, 0x70, 0x33, 0x2D, 0x76, 0x3A, 0x61,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
