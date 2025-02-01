use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850255: FileFormat = FileFormat {
    id: 105_850_255,
    puid: "wikidata/105850255",
    name: "Elite game save (PC)",
    extensions: &["cdr"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x6F, 0x6D, 0x6D, 0x61, 0x6E, 0x64, 0x65, 0x72, 0x20, 0x66, 0x69, 0x6C,
                    0x65, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
