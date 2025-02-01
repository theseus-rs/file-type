use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_55721708: FileFormat = FileFormat {
    id: 55_721_708,
    puid: "wikidata/55721708",
    name: "AmiraMesh Binary Little Endian 2.1 file format",
    extensions: &["am", "amiramesh", "hx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
