use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59537246: FileFormat = FileFormat {
    id: 59_537_246,
    puid: "wikidata/59537246",
    name: "Microsoft OneNote file format",
    extensions: &["one"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
