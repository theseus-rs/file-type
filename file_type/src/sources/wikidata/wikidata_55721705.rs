use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_55721705: FileFormat = FileFormat {
    id: 55_721_705,
    source_type: SourceType::Wikidata,
    name: "AmiraMesh 3D Binary Little Endian 2.0 file format",
    extensions: &["am", "amiramesh", "hx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
