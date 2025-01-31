use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855241: FileFormat = FileFormat {
    id: 105_855_241,
    puid: "wikidata/105855241",
    name: "Felix format spectra",
    extensions: &["fid"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x65, 0x6C, 0x69, 0x78, 0x20, 0x66, 0x6F, 0x72, 0x20, 0x57, 0x69, 0x6E,
                    0x64, 0x6F, 0x77, 0x73, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
