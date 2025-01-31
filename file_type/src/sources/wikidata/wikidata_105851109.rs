use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851109: FileFormat = FileFormat {
    id: 105_851_109,
    puid: "wikidata/105851109",
    name: "TatukGIS Project",
    extensions: &["ttkgp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x54, 0x61, 0x74, 0x75, 0x6B, 0x47, 0x49, 0x53, 0x20, 0x4C, 0x61, 0x79,
                    0x65, 0x72, 0x31, 0x5D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
