use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47012439: FileFormat = FileFormat {
    id: 47_012_439,
    puid: "wikidata/47012439",
    name: "Microsoft Works Document file format",
    extensions: &["bps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
