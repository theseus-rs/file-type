use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857736: FileFormat = FileFormat {
    id: 105_857_736,
    puid: "wikidata/105857736",
    name: "imgdiff patch (v1)",
    extensions: &["patch"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x4D, 0x47, 0x44, 0x49, 0x46, 0x46, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
