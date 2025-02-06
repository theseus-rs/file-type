use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857534: FileFormat = FileFormat {
    id: 105_857_534,
    source_type: SourceType::Wikidata,
    name: "HFS+ / Mac OS Extended disk image",
    extensions: &["hfs"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x2B])],
            },
        }],
    }],
    related_formats: &[],
};
