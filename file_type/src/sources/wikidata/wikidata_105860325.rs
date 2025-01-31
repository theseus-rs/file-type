use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860325: FileFormat = FileFormat {
    id: 105_860_325,
    puid: "wikidata/105860325",
    name: "Helios raw disk image",
    extensions: &["raw"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x48, 0x45, 0x4C, 0x49, 0x4F, 0x53, 0x20, 0x72, 0x61, 0x77, 0x0D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
