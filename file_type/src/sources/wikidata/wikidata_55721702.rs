use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_55721702: FileFormat = FileFormat {
    id: 55_721_702,
    source_type: SourceType::Wikidata,
    name: "AmiraMesh 3D Binary 2.0 file format",
    extensions: &["am", "amiramesh", "hx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
