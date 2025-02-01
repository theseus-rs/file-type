use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857785: FileFormat = FileFormat {
    id: 105_857_785,
    puid: "wikidata/105857785",
    name: "JFD Disk Image (unzipped)",
    extensions: &["jfd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x46, 0x44, 0x49])],
            },
        }],
    }],
    related_formats: &[],
};
