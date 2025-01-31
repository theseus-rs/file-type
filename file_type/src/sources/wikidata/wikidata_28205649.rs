use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205649: FileFormat = FileFormat {
    id: 28_205_649,
    puid: "wikidata/28205649",
    name: "AAI",
    extensions: &["aai"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
