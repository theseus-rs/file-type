use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_83548846: FileFormat = FileFormat {
    id: 83_548_846,
    source_type: SourceType::Wikidata,
    name: "Nearly Raw Raster Data, version 3",
    extensions: &["nrrd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
