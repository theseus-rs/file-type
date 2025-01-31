use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855160: FileFormat = FileFormat {
    id: 105_855_160,
    puid: "wikidata/105855160",
    name: "Frodo SnapShot",
    extensions: &["fss"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x72, 0x6F, 0x64, 0x6F, 0x53, 0x6E, 0x61, 0x70, 0x73, 0x68, 0x6F, 0x74,
                    0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
