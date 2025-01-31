use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975647: FileFormat = FileFormat {
    id: 28_975_647,
    puid: "wikidata/28975647",
    name: "POV-Ray RAW triangle format",
    extensions: &["raw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
