use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27823111: FileFormat = FileFormat {
    id: 27_823_111,
    puid: "wikidata/27823111",
    name: "Bathymetry Attributed Grid",
    extensions: &["bag"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
