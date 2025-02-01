use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83549008: FileFormat = FileFormat {
    id: 83_549_008,
    puid: "wikidata/83549008",
    name: "Nearly Raw Raster Data, version 5",
    extensions: &["nrrd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
