use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83548831: FileFormat = FileFormat {
    id: 83_548_831,
    puid: "wikidata/83548831",
    name: "Nearly Raw Raster Data, version 2",
    extensions: &["nrrd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
