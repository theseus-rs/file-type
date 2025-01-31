use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105853337: FileFormat = FileFormat {
    id: 105_853_337,
    puid: "wikidata/105853337",
    name: "VirtuaNES savestate",
    extensions: &["st0"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x69, 0x72, 0x74, 0x75, 0x61, 0x4E, 0x45, 0x53, 0x20, 0x53, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
