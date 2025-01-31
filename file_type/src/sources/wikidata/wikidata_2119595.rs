use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_2119595: FileFormat = FileFormat {
    id: 2_119_595,
    puid: "wikidata/2119595",
    name: "Wavefront .obj file",
    extensions: &["object", "object"],
    media_types: &["model/obj", "text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
