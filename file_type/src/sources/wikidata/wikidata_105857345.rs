use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857345: FileFormat = FileFormat {
    id: 105_857_345,
    puid: "wikidata/105857345",
    name: "WITCH-DOS Job Control",
    extensions: &["joc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x57, 0x49, 0x54, 0x43, 0x48, 0x2D, 0x44, 0x4F, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
