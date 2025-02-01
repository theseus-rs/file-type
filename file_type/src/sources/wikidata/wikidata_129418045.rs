use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129418045: FileFormat = FileFormat {
    id: 129_418_045,
    puid: "wikidata/129418045",
    name: "GoodData-CL script file",
    extensions: &["gdc"],
    media_types: &["text/x-gooddata-cl"],
    internal_signatures: &[],
    related_formats: &[],
};
