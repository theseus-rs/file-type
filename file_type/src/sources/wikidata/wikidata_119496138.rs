use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119496138: FileFormat = FileFormat {
    id: 119_496_138,
    source_type: SourceType::Wikidata,
    name: "WinFax format",
    extensions: &["wfx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
