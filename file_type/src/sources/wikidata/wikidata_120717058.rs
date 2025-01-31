use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120717058: FileFormat = FileFormat {
    id: 120_717_058,
    puid: "wikidata/120717058",
    name: "DeductionPro 2007 Data File",
    extensions: &["d07"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
