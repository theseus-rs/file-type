use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131703746: FileFormat = FileFormat {
    id: 131_703_746,
    source_type: SourceType::Wikidata,
    name: "xRage hdf file",
    extensions: &["h5rage"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
