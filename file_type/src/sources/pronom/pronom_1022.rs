use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_1022: FileType = FileType {
    file_format: &FileFormat {
        id: 1_022,
        source_type: SourceType::Pronom,
        name: "netCDF-3 Classic",
        extensions: &["nc", "cdf"],
        media_types: &["application/netcdf", "application/x-netcdf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x44, 0x46, 0x01])],
                },
            }],
        }],
        related_formats: &[],
    },
};
