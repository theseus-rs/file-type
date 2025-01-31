use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207555: FileFormat = FileFormat {
    id: 28_207_555,
    puid: "wikidata/28207555",
    name: "eXtended Image File Format, version 3",
    extensions: &["xif"],
    media_types: &["image/vnd.xiff"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x49, 0x2A, 0x00, 0x5C, 0x01, 0x00, 0x00, 0x20, 0x65, 0x58, 0x74, 0x65,
                    0x6E, 0x64, 0x65, 0x64,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
