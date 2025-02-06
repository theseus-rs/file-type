use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27473282: FileFormat = FileFormat {
    id: 27_473_282,
    source_type: SourceType::Wikidata,
    name: "CADRG Legend File",
    extensions: &["lgd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
