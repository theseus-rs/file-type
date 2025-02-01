use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120920266: FileFormat = FileFormat {
    id: 120_920_266,
    puid: "wikidata/120920266",
    name: "Microsoft Money version 1 data",
    extensions: &["mn1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
