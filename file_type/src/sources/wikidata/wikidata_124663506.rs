use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_124663506: FileFormat = FileFormat {
    id: 124_663_506,
    source_type: SourceType::Wikidata,
    name: "Transmission X-Ray Microscopy data format",
    extensions: &["txm", "txrm", "xrm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
