use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_100299227: FileFormat = FileFormat {
    id: 100_299_227,
    puid: "wikidata/100299227",
    name: "Flow Charting file format, version 5",
    extensions: &["fc5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
