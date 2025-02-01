use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_55721671: FileFormat = FileFormat {
    id: 55_721_671,
    puid: "wikidata/55721671",
    name: "AmiraMesh 3D ASCII 2.0 file format",
    extensions: &["am", "amiramesh", "hx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
