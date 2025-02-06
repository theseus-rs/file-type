use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131703407: FileFormat = FileFormat {
    id: 131_703_407,
    source_type: SourceType::Wikidata,
    name: "CONVERGE CFD file format",
    extensions: &["h5"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
