use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975904: FileFormat = FileFormat {
    id: 28_975_904,
    puid: "wikidata/28975904",
    name: "Specified points for body pressure file",
    extensions: &["bpi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
