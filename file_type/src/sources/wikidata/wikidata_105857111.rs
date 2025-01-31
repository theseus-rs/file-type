use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857111: FileFormat = FileFormat {
    id: 105_857_111,
    puid: "wikidata/105857111",
    name: "NTv2 Standard Binary Grid Shift",
    extensions: &["gsb"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4E, 0x55, 0x4D, 0x5F, 0x4F, 0x52, 0x45, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
