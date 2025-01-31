use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975862: FileFormat = FileFormat {
    id: 28_975_862,
    puid: "wikidata/28975862",
    name: "OOGL Bezier Surface BBP",
    extensions: &["bbp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
