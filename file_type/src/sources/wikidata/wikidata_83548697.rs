use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83548697: FileFormat = FileFormat {
    id: 83_548_697,
    source_type: SourceType::Wikidata,
    name: "Nearly Raw Raster Data, version 1",
    extensions: &["nrrd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
