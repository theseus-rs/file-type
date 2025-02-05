use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28346237: FileFormat = FileFormat {
    id: 28_346_237,
    source_type: SourceType::Wikidata,
    name: "TTDDD",
    extensions: &["ttd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
