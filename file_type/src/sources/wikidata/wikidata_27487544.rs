use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27487544: FileFormat = FileFormat {
    id: 27_487_544,
    puid: "wikidata/27487544",
    name: "Shapefile codepage file",
    extensions: &["cpg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
