use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_55721705: FileFormat = FileFormat {
    id: 55_721_705,
    puid: "wikidata/55721705",
    name: "AmiraMesh 3D Binary Little Endian 2.0 file format",
    extensions: &["am", "amiramesh", "hx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
