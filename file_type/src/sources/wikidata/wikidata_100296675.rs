use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100296675: FileFormat = FileFormat {
    id: 100_296_675,
    source_type: SourceType::Wikidata,
    name: "Flow Charting file format, version I-II+",
    extensions: &["cht"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
