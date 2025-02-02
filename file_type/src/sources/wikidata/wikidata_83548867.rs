use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_83548867: FileFormat = FileFormat {
    id: 83_548_867,
    source_type: SourceType::Wikidata,
    name: "Nearly Raw Raster Data, version 4",
    extensions: &["nrrd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
