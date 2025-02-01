use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131860302: FileFormat = FileFormat {
    id: 131_860_302,
    puid: "wikidata/131860302",
    name: "MNI transformation file format",
    extensions: &["xfm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
