use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27487495: FileFormat = FileFormat {
    id: 27_487_495,
    puid: "wikidata/27487495",
    name: "Shapefile spatial index of features part 2",
    extensions: &["fbx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
