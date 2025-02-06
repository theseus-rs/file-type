use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_95994804: FileFormat = FileFormat {
    id: 95_994_804,
    source_type: SourceType::Wikidata,
    name: "Spatial Data Transfer Standard format",
    extensions: &["ddf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
