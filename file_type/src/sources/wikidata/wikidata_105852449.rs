use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852449: FileFormat = FileFormat {
    id: 105_852_449,
    puid: "wikidata/105852449",
    name: "AmiAtlas Share data",
    extensions: &["share"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x4D, 0x49, 0x47, 0x41, 0x5F, 0x41, 0x54, 0x4C, 0x41, 0x53, 0x5F, 0x53,
                    0x48, 0x41, 0x52, 0x45, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
