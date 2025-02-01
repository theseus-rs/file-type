use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858086: FileFormat = FileFormat {
    id: 105_858_086,
    puid: "wikidata/105858086",
    name: "PPrint Image",
    extensions: &["ima"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x4D, 0x41, 0x47, 0x45, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
