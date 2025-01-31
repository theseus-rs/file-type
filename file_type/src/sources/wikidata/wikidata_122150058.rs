use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122150058: FileFormat = FileFormat {
    id: 122_150_058,
    puid: "wikidata/122150058",
    name: "TaxAct 2007 Tax Return File",
    extensions: &["ta7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
