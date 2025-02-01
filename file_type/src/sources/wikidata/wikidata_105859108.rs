use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859108: FileFormat = FileFormat {
    id: 105_859_108,
    puid: "wikidata/105859108",
    name: "Speccy eXtended Graphics bitmap",
    extensions: &["sxg"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7F, 0x53, 0x58, 0x47])],
            },
        }],
    }],
    related_formats: &[],
};
