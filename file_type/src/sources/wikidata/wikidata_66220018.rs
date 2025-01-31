use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_66220018: FileFormat = FileFormat {
    id: 66_220_018,
    puid: "wikidata/66220018",
    name: "Adobe GoLive Actions file format",
    extensions: &["actions"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
