use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120717835: FileFormat = FileFormat {
    id: 120_717_835,
    puid: "wikidata/120717835",
    name: "DeductionPro 2006 Data File",
    extensions: &["d06"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
