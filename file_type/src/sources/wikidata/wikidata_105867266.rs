use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867266: FileFormat = FileFormat {
    id: 105_867_266,
    puid: "wikidata/105867266",
    name: "Noteworthy Composer data file",
    extensions: &["nwc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5B, 0x4E, 0x57, 0x5A, 0x5D, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
