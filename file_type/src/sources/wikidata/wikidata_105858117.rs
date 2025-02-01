use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858117: FileFormat = FileFormat {
    id: 105_858_117,
    puid: "wikidata/105858117",
    name: "VTrucco disk image",
    extensions: &["vtr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x56, 0x54, 0x72, 0x75, 0x63, 0x63, 0x6F, 0x01, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
