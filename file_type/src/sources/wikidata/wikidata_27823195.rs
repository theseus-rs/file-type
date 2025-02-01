use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27823195: FileFormat = FileFormat {
    id: 27_823_195,
    puid: "wikidata/27823195",
    name: "Binary Terrain External Projection file",
    extensions: &["prj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
