use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83548831: FileFormat = FileFormat {
    id: 83_548_831,
    source_type: SourceType::Wikidata,
    name: "Nearly Raw Raster Data, version 2",
    extensions: &["nrrd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
