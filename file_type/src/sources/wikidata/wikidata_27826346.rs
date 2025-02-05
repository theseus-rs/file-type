use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27826346: FileFormat = FileFormat {
    id: 27_826_346,
    source_type: SourceType::Wikidata,
    name: "Reduced resolution dataset file",
    extensions: &["rrd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
