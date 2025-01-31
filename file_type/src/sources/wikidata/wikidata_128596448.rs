use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128596448: FileFormat = FileFormat {
    id: 128_596_448,
    puid: "wikidata/128596448",
    name: "Alloy file format",
    extensions: &["als"],
    media_types: &["text/x-alloy"],
    internal_signatures: &[],
    related_formats: &[],
};
