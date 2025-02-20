use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867343: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_343,
        source_type: SourceType::Wikidata,
        name: "NetCDF Network Common Data Form",
        extensions: &["cdf", "nc"],
        media_types: &["application/netcdf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x44, 0x46])],
                },
            }],
        }],
        related_formats: &[],
    },
};
