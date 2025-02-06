use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_55721702: FileFormat = FileFormat {
    id: 55_721_702,
    source_type: SourceType::Wikidata,
    name: "AmiraMesh 3D Binary 2.0 file format",
    extensions: &["am", "amiramesh", "hx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
