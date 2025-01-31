use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857117: FileFormat = FileFormat {
    id: 105_857_117,
    puid: "wikidata/105857117",
    name: "GIMS Graphical Text data",
    extensions: &["gxt"],
    media_types: &["text/ini"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x49, 0x6E, 0x66, 0x6F, 0x5D, 0x0D,
                    0x0A, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x5F, 0x76, 0x65, 0x72, 0x3D, 0x31,
                    0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
