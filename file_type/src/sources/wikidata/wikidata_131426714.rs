use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131426714: FileFormat = FileFormat {
    id: 131_426_714,
    puid: "wikidata/131426714",
    name: "X++ source code file format",
    extensions: &["xpp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
