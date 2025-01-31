use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_18812775: FileFormat = FileFormat {
    id: 18_812_775,
    puid: "wikidata/18812775",
    name: "VTK format",
    extensions: &["vtk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
