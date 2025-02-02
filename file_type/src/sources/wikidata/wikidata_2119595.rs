use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_2119595: FileFormat = FileFormat {
    id: 2_119_595,
    source_type: SourceType::Wikidata,
    name: "Wavefront .obj file",
    extensions: &["object"],
    media_types: &["model/obj", "text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
