use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28771267: FileFormat = FileFormat {
    id: 28_771_267,
    puid: "wikidata/28771267",
    name: "MLM",
    extensions: &["mlm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
