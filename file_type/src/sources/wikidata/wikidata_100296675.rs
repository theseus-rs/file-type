use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100296675: FileFormat = FileFormat {
    id: 100_296_675,
    puid: "wikidata/100296675",
    name: "Flow Charting file format, version I-II+",
    extensions: &["cht"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
