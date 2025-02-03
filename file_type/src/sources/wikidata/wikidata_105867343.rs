use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105867343: FileFormat = FileFormat {
    id: 105_867_343,
    source_type: SourceType::Wikidata,
    name: "NetCDF Network Common Data Form",
    extensions: &["cdf", "nc"],
    media_types: &["application/netcdf"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x44, 0x46])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x44, 0x46])],
                },
            }],
        },
    ],
    related_formats: &[],
};
