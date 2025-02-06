use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50308914: FileFormat = FileFormat {
    id: 50_308_914,
    source_type: SourceType::Wikidata,
    name: "OMNIC Spectral Data File",
    extensions: &["spa"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
