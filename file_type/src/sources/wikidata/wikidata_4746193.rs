use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4746193: FileFormat = FileFormat {
    id: 4_746_193,
    puid: "wikidata/4746193",
    name: "Amiga Disk File",
    extensions: &["adf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x4F, 0x53])],
            },
        }],
    }],
    related_formats: &[],
};
